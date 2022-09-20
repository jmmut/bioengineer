mod networks;
pub mod robots;

use crate::now;
use crate::screen::gui::GuiActions;
use crate::world::game_state::networks::Networks;
use crate::world::game_state::robots::{
    is_position_actionable, move_robot_to_position, move_robot_to_tasks, reachable_positions, Robot,
};
use crate::world::game_state::GameGoalState::Started;
use crate::world::map::fluids::{FluidMode, Fluids};
use crate::world::map::transform_cells::Transformation;
use crate::world::map::CellIndex;
use crate::world::map::Map;
use std::collections::{HashSet, VecDeque};

const DEFAULT_PROFILE_ENABLED: bool = false;
const DEFAULT_ADVANCING_FLUIDS: bool = false;
const DEFAULT_ADVANCE_FLUID_EVERY_N_FRAMES: i32 = 10;
const DEFAULT_ADVANCE_ROBOTS_EVERY_N_FRAMES: i32 = 15;

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum GameGoalState {
    Started,
    Finished,
    PostFinished,
}

pub struct GameState {
    pub frame_index: i32,
    pub previous_frame_ts: f64,
    pub current_frame_ts: f64,
    pub map: Map,
    pub advancing_fluids: bool,
    pub advance_fluid_every_n_frames: i32,
    pub advance_robots_every_n_frames: i32,
    pub fluids: Fluids,
    pub profile: bool, // TODO: move to DrawingState?
    pub robots: Vec<Robot>,
    pub task_queue: VecDeque<Task>,
    pub networks: Networks,
    pub goal_state: GameGoalState,
}

impl GameState {
    pub fn new() -> GameState {
        let mut map = Map::new();
        map.regenerate();
        let profile = DEFAULT_PROFILE_ENABLED;
        let mut fluids = Fluids::new(FluidMode::InStages);
        fluids.set_profile(profile);
        let ship_position = map.get_ship_position();
        let robots = match ship_position {
            Option::None => vec![],
            Option::Some(position) => vec![Robot { position }],
        };
        GameState {
            frame_index: 0,
            previous_frame_ts: now() - 1.0,
            current_frame_ts: now(),
            map,
            advancing_fluids: DEFAULT_ADVANCING_FLUIDS,
            advance_fluid_every_n_frames: DEFAULT_ADVANCE_FLUID_EVERY_N_FRAMES,
            advance_robots_every_n_frames: DEFAULT_ADVANCE_ROBOTS_EVERY_N_FRAMES,
            fluids,
            profile,
            robots,
            task_queue: VecDeque::new(),
            networks: Networks::new(),
            goal_state: Started,
        }
    }

    pub fn update_with_gui_actions(&mut self, gui_actions: &GuiActions) {
        self.update_task_queue(gui_actions);

        if gui_actions.input.toggle_profiling {
            self.profile = !self.profile;
        }
        if gui_actions.input.toggle_fluids {
            self.advancing_fluids = !self.advancing_fluids;
        }
        if self.should_advance_fluids_this_frame(gui_actions) {
            self.fluids.advance(&mut self.map);
        }

        if gui_actions.input.regenerate_map {
            self.map.regenerate();
        }
        self.goal_state = gui_actions.next_game_goal_state.unwrap_or(self.goal_state);
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

        if self.should_advance_robots_this_frame() {
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
                };
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

    fn queue_transformation(&mut self, transformation_task: TransformationTask) {
        self.task_queue
            .push_back(Task::Transform(transformation_task));
    }
    fn queue_movement(&mut self, destination: CellIndex) {
        self.task_queue.push_back(Task::Movement(destination));
    }

    fn move_robots(&mut self) {
        for robot in &mut self.robots {
            let movement_opt = move_robot_to_tasks(robot.position, &self.task_queue, &self.map);
            if let Option::Some(movement) = movement_opt {
                robot.position += movement;
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
                if is_position_actionable(
                    self.map.get_cell(robot.position).tile_type,
                    &robot.position,
                    &reachable_position,
                ) {
                    let cell = self.map.get_cell_mut(reachable_position);
                    transform.transformation.apply(cell);
                    self.networks.add(reachable_position, cell.tile_type);
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

    fn should_advance_robots_this_frame(&mut self) -> bool {
        let should_process_frame = self.frame_index % self.advance_robots_every_n_frames == 0;
        should_process_frame
    }

    fn should_advance_fluids_this_frame(&mut self, gui_actions: &GuiActions) -> bool {
        if gui_actions.input.single_fluid {
            return true;
        } else {
            if self.advancing_fluids {
                let should_process_frame =
                    self.frame_index % self.advance_fluid_every_n_frames == 0;
                return should_process_frame;
            }
        }
        return false;
    }

    pub fn advance_frame(&mut self) {
        self.frame_index = (self.frame_index + 1) % 1000;
        self.previous_frame_ts = self.current_frame_ts;
        self.current_frame_ts = now();
    }
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
