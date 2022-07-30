pub mod robots;

use super::map::Map;
use crate::drawing::Drawing;
use crate::game_state::robots::{
    is_position_reachable, move_robot_to_position, move_robot_to_tasks, reachable_positions, Robot,
};
use crate::gui::GuiActions;
use crate::map::fluids::{FluidMode, Fluids};
use crate::map::transform_cells::Transformation;
use crate::map::CellIndex;
use crate::now;
use std::collections::{HashSet, VecDeque};

const DEFAULT_PROFILE_ENABLED: bool = false;
const DEFAULT_ADVANCING_FLUIDS: bool = false;
const DEFAULT_ADVANCE_FLUID_EVERY_N_FRAMES: i32 = 1;
const DEFAULT_ADVANCE_ROBOTS_EVERY_N_FRAMES: i32 = 15;

pub struct GameState {
    pub frame_index: i32,
    pub previous_frame_ts: f64,
    pub current_frame_ts: f64,
    pub map: Map,
    pub drawing: Drawing,
    pub advancing_fluids: bool,
    pub advance_fluid_every_n_frames: i32,
    pub advance_robots_every_n_frames: i32,
    pub fluids: Fluids,
    pub profile: bool,
    pub robots: Vec<Robot>,
    pub task_queue: VecDeque<Task>,
    pub movement_queue: VecDeque<CellIndex>,
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
            drawing: Drawing::new(),
            advancing_fluids: DEFAULT_ADVANCING_FLUIDS,
            advance_fluid_every_n_frames: DEFAULT_ADVANCE_FLUID_EVERY_N_FRAMES,
            advance_robots_every_n_frames: DEFAULT_ADVANCE_ROBOTS_EVERY_N_FRAMES,
            fluids,
            profile,
            robots,
            task_queue: VecDeque::new(),
            movement_queue: VecDeque::new(),
        }
    }

    pub fn update_with_gui_actions(&mut self, gui_actions: &GuiActions) {
        self.update_task_and_movement_queues(gui_actions);

        if gui_actions.input.toggle_fluids {
            self.advancing_fluids = !self.advancing_fluids;
        }
        if self.should_advance_fluids_this_frame(&gui_actions) {
            self.fluids.advance(&mut self.map);
        }

        if gui_actions.input.regenerate_map {
            self.map.regenerate();
        }
    }

    fn update_task_and_movement_queues(&mut self, gui_actions: &GuiActions) {
        if let Option::Some(do_now_task) = gui_actions.do_now_task {
            let task = self.task_queue.remove(do_now_task);
            self.task_queue.push_front(task.unwrap());
        }
        if let Option::Some(do_now_movement) = gui_actions.do_now_movement {
            let movement = self.movement_queue.remove(do_now_movement);
            self.movement_queue.push_front(movement.unwrap());
        }
        if let Option::Some(cancel_task) = gui_actions.cancel_task {
            self.task_queue.remove(cancel_task);
        }
        if let Option::Some(cancel_movement) = gui_actions.cancel_movement {
            self.movement_queue.remove(cancel_movement);
        }

        if let Option::Some(transformation) = gui_actions.selected_cell_transformation {
            self.movement_queue.clear();
            self.queue_transformation(transformation);
        }
        if let Option::Some(target_cell) = gui_actions.robot_movement {
            self.task_queue.clear();
            self.movement_queue.push_back(target_cell);
        }

        if self.should_advance_robots_this_frame() {
            if let Option::Some(movement_target) = self.movement_queue.front().cloned() {
                self.move_robots_to_position(&movement_target)
            } else {
                self.transform_cells_if_robots_can_do_so();
                self.move_robots();
            }
        }
    }

    fn move_robots_to_position(&mut self, movement_target: &CellIndex) {
        for robot in &mut self.robots {
            let movement_opt = move_robot_to_position(robot.position, &movement_target, &self.map);
            if let Option::Some(movement) = movement_opt {
                robot.position += movement;
                if robot.position == *movement_target {
                    self.movement_queue.pop_front();
                }
            } else {
                self.movement_queue.pop_front();
            }
        }
    }

    fn queue_transformation(&mut self, transformation: Transformation) {
        self.task_queue.push_back(Task {
            to_transform: self.drawing.highlighted_cells.clone(),
            transformation,
        });
    }

    fn move_robots(&mut self) {
        for robot in &mut self.robots {
            let movement_opt = move_robot_to_tasks(robot.position, &self.task_queue, &self.map);
            if let Option::Some(movement) = movement_opt {
                robot.position += movement;
            }
        }
    }

    fn transform_cells_if_robots_can_do_so(&mut self) {
        for robot in &self.robots.clone() {
            self.transform_single_cell_if_robot_can_do_so(robot)
        }
    }

    fn transform_single_cell_if_robot_can_do_so(&mut self, robot: &Robot) {
        let task_opt = self.task_queue.front_mut();
        if let Option::Some(task) = task_opt {
            for reachable_pos_diff in reachable_positions() {
                let reachable_position = robot.position + reachable_pos_diff;
                let transformation_here = task.to_transform.take(&reachable_position);
                if transformation_here.is_some() {
                    if is_position_reachable(
                        self.map.get_cell(robot.position).tile_type,
                        &robot.position,
                        &reachable_position,
                    ) {
                        task.transformation
                            .apply(self.map.get_cell_mut(reachable_position));
                        if task.to_transform.len() == 0 {
                            self.task_queue.pop_front();
                        }
                        // we only want to transform 1 cell, so do an early return
                        return;
                    }
                }
            }
        }
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
    pub fn get_drawing(&self) -> &Drawing {
        &self.drawing
    }
    pub fn get_drawing_mut(&mut self) -> &mut Drawing {
        &mut self.drawing
    }
}

pub struct Task {
    pub to_transform: HashSet<CellIndex>,
    pub transformation: Transformation,
}
