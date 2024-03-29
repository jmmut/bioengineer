pub mod fluids;
pub mod game_state;
mod gameplay_tests;
pub mod map;
mod networks;
pub mod robots;

use std::collections::{HashSet, VecDeque};

use crate::screen::gui::format_units::format_age;
use fluids::FluidMode;
use fluids::Fluids;
use game_state::get_goal_air_cleaned;
use game_state::GameState;
use map::transform_cells::Transformation;
use map::CellIndex;
use map::Map;
use networks::Networks;
use robots::Robot;
use robots::{
    is_tile_actionable, move_robot_to_position, move_robot_to_tasks, reachable_positions,
};

use crate::screen::gui::gui_actions::GuiActions;
use crate::world::game_state::{DEFAULT_ADVANCING_FLUIDS, DEFAULT_PROFILE_ENABLED};
use crate::world::map::cell::{ages, transition_aging_tile};
use crate::world::map::{Cell, TileType};

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
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum GameGoalState {
    Started,
    ReachedProduction,
    Finished(AgeInMinutes),
    PostFinished,
}

impl World {
    #[allow(unused)]
    pub fn new() -> Self {
        Self::new_with_options(DEFAULT_PROFILE_ENABLED, DEFAULT_ADVANCING_FLUIDS)
    }

    pub fn new_with_options(profile: bool, fluids: bool) -> Self {
        let game_state = GameState::new(fluids);
        let mut map = Map::new();
        map.regenerate();
        let ship_position = map.get_ship_position();
        let fluids = Fluids::new(FluidMode::InStages);
        let robots = Self::reset_robots(ship_position);
        let mut world = World {
            map,
            fluids,
            robots,
            task_queue: VecDeque::new(),
            networks: Networks::new(),
            aging_tiles: HashSet::new(),
            life: HashSet::new(),
            game_state,
            goal_state: GameGoalState::Started,
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
    pub fn update(&mut self, gui_actions: GuiActions) -> bool {
        self.update_with_gui_actions(&gui_actions);
        self.advance_frame();
        gui_actions.should_continue()
    }

    pub fn update_with_gui_actions(&mut self, gui_actions: &GuiActions) {
        if gui_actions.toggle_profiling {
            self.set_profile(!self.game_state.profile);
        }

        self.game_state.update_with_gui_actions(gui_actions);

        self.update_task_queue(gui_actions);

        if self.game_state.should_advance_fluids_this_frame() {
            self.fluids.advance(&mut self.map);
        }

        if gui_actions.regenerate_map {
            self.map.regenerate();
            self.networks.clear();
            self.robots = Self::reset_robots(self.map.get_ship_position());
            self.task_queue.clear();
        }
        self.networks.update();
        if self.game_state.should_age_this_frame() {
            self.age_tiles();
        }
        self.update_goal_state(gui_actions);
    }

    fn reset_robots(ship_position: Option<CellIndex>) -> Vec<Robot> {
        match ship_position {
            Option::None => vec![],
            Option::Some(position) => vec![Robot { position }],
        }
    }

    fn update_task_queue(&mut self, gui_actions: &GuiActions) {
        if let Option::Some(do_now_task) = gui_actions.do_now_task {
            let task = self.task_queue.remove(do_now_task);
            self.task_queue.push_front(task.unwrap());
        }
        if let Option::Some(cancel_task) = gui_actions.cancel_task {
            self.task_queue.remove(cancel_task);
        }

        if let Option::Some(transformation_task) = gui_actions.selected_cell_transformation.clone()
        {
            self.queue_transformation(transformation_task);
        }
        if let Option::Some(target_cell) = gui_actions.robot_movement {
            self.queue_movement(target_cell);
        }

        if self.game_state.should_advance_robots_this_frame() {
            self.advance_robots_task_queue();
        }
    }

    fn queue_transformation(&mut self, transformation_task: TransformationTask) {
        self.task_queue
            .push_back(Task::Transform(transformation_task));
    }
    fn queue_movement(&mut self, destination: CellIndex) {
        self.task_queue.push_back(Task::Movement(destination));
    }

    fn advance_robots_task_queue(&mut self) {
        if let Option::Some(task) = self.task_queue.pop_front() {
            match task {
                Task::Transform(transform) => {
                    let remaining = self.transform_cells_if_robots_can_do_so(transform);
                    if let Option::Some(remaining_task) = remaining {
                        self.task_queue.push_front(Task::Transform(remaining_task));
                    }
                    self.move_robots();
                }
                Task::Movement(movement) => {
                    let remaining = self.move_robots_to_position(&movement);
                    if let Option::Some(remaining_task) = remaining {
                        self.task_queue.push_front(Task::Movement(remaining_task));
                    }
                }
            }
        }
    }

    fn transform_cells_if_robots_can_do_so(
        &mut self,
        transform: TransformationTask,
    ) -> Option<TransformationTask> {
        let mut transform_opt = Option::Some(transform);
        for robot in &self.robots.clone() {
            if let Option::Some(transform) = transform_opt {
                transform_opt = self.transform_single_cell_if_robot_can_do_so(robot, transform);
            }
        }
        return transform_opt;
    }

    /// Returns None: task finished. Some: task partially advanced.
    fn transform_single_cell_if_robot_can_do_so(
        &mut self,
        robot: &Robot,
        mut transform: TransformationTask,
    ) -> Option<TransformationTask> {
        for reachable_pos_diff in reachable_positions() {
            let reachable_position = robot.position + reachable_pos_diff;
            let transformation_here = transform.to_transform.take(&reachable_position);
            if transformation_here.is_some() {
                if is_tile_actionable(
                    self.map.get_cell(robot.position).tile_type,
                    &robot.position,
                    &reachable_position,
                ) {
                    let cell = self.map.get_cell_mut(reachable_position);
                    transform.transformation.apply(cell);
                    self.networks.add(reachable_position, cell.tile_type);
                    if ages(cell.tile_type) {
                        self.aging_tiles.insert(reachable_position);
                        self.life.insert(reachable_position);
                    } else {
                        self.aging_tiles.remove(&reachable_position);
                        self.life.remove(&reachable_position);
                    }
                    if transform.to_transform.len() == 0 {
                        // no need to reinsert this
                        return Option::None;
                    }
                    // we only want to transform 1 cell, so do an early return with the remaining
                    // positions of this task
                    return Option::Some(transform);
                } else {
                    transform.to_transform.insert(reachable_position);
                }
            }
        }
        // No actions done. Task still pending.
        return Option::Some(transform);
    }

    fn move_robots(&mut self) {
        for robot in &mut self.robots {
            let movement_opt = move_robot_to_tasks(robot.position, &self.task_queue, &self.map);
            if let Option::Some(movement) = movement_opt {
                robot.position += movement;
            }
        }
    }

    /// returns Some: more movement needed. None: destination reached.
    fn move_robots_to_position(&mut self, movement_target: &CellIndex) -> Option<CellIndex> {
        for robot in &mut self.robots {
            let movement_opt = move_robot_to_position(robot.position, movement_target, &self.map);
            if let Option::Some(movement) = movement_opt {
                robot.position += movement;
                if robot.position == *movement_target {
                    return Option::None;
                }
            } else {
                return Option::None;
            }
        }
        return Option::Some(*movement_target);
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
            self.goal_state = GameGoalState::Started;
        }
        match self.goal_state {
            GameGoalState::Finished(_) | GameGoalState::PostFinished => {
                self.goal_state = gui_actions.next_game_goal_state.unwrap_or(self.goal_state);
            }
            _ => transition_goal_state(
                &mut self.goal_state,
                &self.networks,
                self.life.len(),
                self.age_in_minutes,
            ),
        }
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
        if networks.len() == 0 && life_count >= LIFE_COUNT_REQUIRED_FOR_WINNING {
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
