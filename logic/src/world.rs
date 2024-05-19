pub mod fluids;
pub mod game_state;
pub mod gameplay_tests;
pub mod map;
pub mod networks;
pub mod robots;

use std::collections::{HashSet, VecDeque};

use crate::scene::GameLoopState;
use crate::screen::gui::format_units::format_age;
use fluids::FluidMode;
use fluids::Fluids;
use game_state::get_goal_air_cleaned;
use game_state::GameState;
use map::transform_cells::Transformation;
use map::transformation_rules::TransformationRules;
use map::CellIndex;
use map::Map;
use networks::Networks;
use robots::Robot;

use crate::screen::gui::gui_actions::GuiActions;
use crate::world::game_state::{DEFAULT_ADVANCING_FLUIDS, DEFAULT_PROFILE_ENABLED};
use crate::world::map::cell::{ages, transition_aging_tile};
use crate::world::map::transform_cells::TransformationFailure;
use crate::world::map::{Cell, MapType, TileType, DEFAULT_MAP_TYPE};

type AgeInMinutes = i64;

pub const LIFE_COUNT_REQUIRED_FOR_WINNING: usize = 50;

pub struct World {
    pub map: Map,
    pub fluids: Fluids,
    pub robots: Vec<Robot>,
    pub task_queue: VecDeque<Task>,
    pub networks: Networks,
    pub aging_tiles: HashSet<CellIndex>,
    pub life: HashSet<CellIndex>,
    pub game_state: GameState,
    pub goal_state: GameGoalState,
    pub age_in_minutes: AgeInMinutes,
}

#[derive(Clone)]
pub enum Task {
    Transform(TransformationTask),
    Movement(CellIndex),
}

#[derive(Clone)]
pub struct TransformationTask {
    pub to_transform: HashSet<CellIndex>,
    pub transformation: Transformation,
    pub blocked_because: Option<HashSet<TransformationFailure>>,
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum GameGoalState {
    InitialDialog,
    Started,
    ReachedProduction,
    Finished(AgeInMinutes),
    PostFinished,
}

impl World {
    #[allow(unused)]
    pub fn new() -> Self {
        Self::new_with_options(
            DEFAULT_PROFILE_ENABLED,
            DEFAULT_ADVANCING_FLUIDS,
            DEFAULT_MAP_TYPE,
        )
    }

    pub fn new_with_options(profile: bool, fluids: bool, map_type: MapType) -> Self {
        let game_state = GameState::new(fluids);
        let map = Map::new_generated(map_type);
        let ship_position = map.get_ship_position();
        let fluids = Fluids::new(FluidMode::InStages);
        let robots = Self::reset_robots(ship_position);
        let mut world = World {
            map,
            fluids,
            robots,
            task_queue: VecDeque::new(),
            networks: Networks::new(ship_position.unwrap()),
            aging_tiles: HashSet::new(),
            life: HashSet::new(),
            game_state,
            goal_state: GameGoalState::InitialDialog,
            age_in_minutes: 0,
        };
        world.set_profile(profile);
        world
    }

    pub fn set_profile(&mut self, profile: bool) {
        self.game_state.profile = profile;
        self.fluids.set_profile(profile);
    }

    /// returns if the game should do another iteration
    pub fn update(&mut self, gui_actions: GuiActions) -> GameLoopState {
        let should_continue = self.update_with_gui_actions(&gui_actions);
        self.advance_frame();
        should_continue
    }

    pub fn update_with_gui_actions(&mut self, gui_actions: &GuiActions) -> GameLoopState {
        if gui_actions.toggle_profiling {
            self.set_profile(!self.game_state.profile);
        }

        self.game_state.update_with_gui_actions(gui_actions);

        self.update_task_queue(gui_actions);

        if self.game_state.should_advance_fluids_this_frame() {
            self.fluids.advance(&mut self.map);
        }

        let should_continue = if gui_actions.regenerate_map {
            self.map.regenerate();
            self.networks.clear();
            self.robots = Self::reset_robots(self.map.get_ship_position());
            self.task_queue.clear();
            GameLoopState::ShouldRepositionCamera
        } else {
            gui_actions.should_continue()
        };
        self.networks.update();
        if self.game_state.should_age_this_frame() {
            self.age_tiles();
        }
        self.update_goal_state(gui_actions);
        should_continue
    }

    fn reset_robots(ship_position: Option<CellIndex>) -> Vec<Robot> {
        match ship_position {
            Option::None => vec![],
            Option::Some(position) => vec![Robot { position }],
        }
    }

    fn update_task_queue(&mut self, gui_actions: &GuiActions) {
        if let Option::Some(cancel_task) = gui_actions.cancel_task {
            self.task_queue.remove(cancel_task);
        }

        if let Option::Some(transformation_task) = gui_actions.selected_cell_transformation.clone()
        {
            self.queue_transformation(transformation_task);
        }

        if self.game_state.should_advance_robots_this_frame() {
            self.advance_construction_task_queue();
            // self.advance_robots_task_queue();
        }
    }

    fn queue_transformation(&mut self, transformation_task: TransformationTask) {
        self.task_queue
            .push_back(Task::Transform(transformation_task));
    }

    #[allow(unused)]
    fn queue_movement(&mut self, destination: CellIndex) {
        self.task_queue.push_back(Task::Movement(destination));
    }

    fn advance_construction_task_queue(&mut self) {
        if let Some(task) = self.task_queue.pop_back() {
            match task {
                Task::Transform(task) => {
                    if let Some(remaining_task) = self.try_build(task) {
                        self.task_queue.push_back(remaining_task);
                    }
                }
                Task::Movement(_) => {}
            }
        }
    }

    /// returns a pending task for whatever is left from the given transformation,
    /// or None if the transformation was finished
    fn try_build(&mut self, task: TransformationTask) -> Option<Task> {
        let TransformationTask {
            to_transform,
            transformation,
            ..
        } = task;
        let mut remaining = HashSet::new();
        let mut reasons = HashSet::new();
        let mut adjacent = Vec::<CellIndex>::new();
        let mut add_reason = |position, reason| {
            remaining.insert(position);
            reasons.insert(reason);
        };
        for pos_to_transform in to_transform {
            if self.networks.is_adjacent_to_ship_network(pos_to_transform) {
                adjacent.push(pos_to_transform);
            } else {
                add_reason(pos_to_transform, TransformationFailure::OutOfShipReach);
            }
        }
        for pos_to_transform in adjacent {
            let rules =
                TransformationRules::new(pos_to_transform, transformation.new_tile_type, &self.map);

            if let Some(reason) = rules.is_forbidden() {
                add_reason(pos_to_transform, reason);
            } else if let Some(reason) = self.try_update_network(transformation, pos_to_transform) {
                add_reason(pos_to_transform, reason);
            }
        }
        if remaining.len() > 0 {
            Some(Task::Transform(TransformationTask::new_with_reason(
                remaining,
                transformation,
                Some(reasons),
            )))
        } else {
            None
        }
    }

    /// returns Some(reason) if the update failed, or None if it was ok
    fn try_update_network(
        &mut self,
        transformation: Transformation,
        pos_to_transform: CellIndex,
    ) -> Option<TransformationFailure> {
        let cell = self.map.get_cell_mut(pos_to_transform);
        let mut cell_copy = cell.clone();
        transformation.apply(&mut cell_copy);
        let was_transformed =
            self.networks
                .add_with_reason(pos_to_transform, cell_copy.tile_type, cell.tile_type);
        if was_transformed == None {
            *cell = cell_copy;
            if ages(cell.tile_type) {
                // TODO: is_alive(). otherwise it doesn't make sense to have aging_tiles and life as separate variables
                self.aging_tiles.insert(pos_to_transform);
                self.life.insert(pos_to_transform);
            } else {
                self.aging_tiles.remove(&pos_to_transform);
                self.life.remove(&pos_to_transform);
            }
        }
        was_transformed
    }

    fn age_tiles(&mut self) {
        for cell_index in &self.aging_tiles {
            let cell = self.map.get_cell_mut(cell_index.clone());
            let died = age_tile(cell, self.goal_state);
            if died {
                self.life.remove(cell_index);
            }
        }
    }

    fn update_goal_state(&mut self, gui_actions: &GuiActions) {
        if gui_actions.reset_quantities {
            self.networks.reset_production();
            self.age_in_minutes = 0;
            self.goal_state = GameGoalState::InitialDialog;
        }
        if let Some(new_state) = gui_actions.next_game_goal_state {
            self.goal_state = new_state;
        }
        transition_goal_state(
            &mut self.goal_state,
            &self.networks,
            self.life.len(),
            self.age_in_minutes,
        );
    }

    fn advance_frame(&mut self) {
        self.game_state.advance_frame();
        if self.game_state.frame_index % 60 == 0 {
            self.age_in_minutes += 1;
        }
    }

    pub fn get_age_str(&self) -> String {
        format_age(self.age_in_minutes)
    }
}

fn transition_goal_state(
    current: &mut GameGoalState,
    networks: &Networks,
    life_count: usize,
    age: AgeInMinutes,
) {
    if *current == GameGoalState::Started {
        if networks.get_total_air_cleaned() >= get_goal_air_cleaned() {
            *current = GameGoalState::ReachedProduction;
        }
    } else if *current == GameGoalState::ReachedProduction {
        if networks.get_non_ship_machine_count() == 0
            && life_count >= LIFE_COUNT_REQUIRED_FOR_WINNING
        {
            *current = GameGoalState::Finished(age);
        }
    }
}

/// Returns true if the cell died
fn age_tile(cell: &mut Cell, goal_state: GameGoalState) -> bool {
    match goal_state {
        GameGoalState::Started => {
            cell.health -= 1;
            if cell.health <= 0 {
                transition_aging_tile(cell);
            }
        }
        _ => (),
    }
    return cell.tile_type == TileType::TreeDead;
}

impl TransformationTask {
    pub fn new(to_transform: HashSet<CellIndex>, transformation: Transformation) -> Self {
        Self {
            to_transform,
            transformation,
            blocked_because: None,
        }
    }
    pub fn new_with_reason(
        to_transform: HashSet<CellIndex>,
        transformation: Transformation,
        blocked_because: Option<HashSet<TransformationFailure>>,
    ) -> Self {
        Self {
            to_transform,
            transformation,
            blocked_because,
        }
    }
}
