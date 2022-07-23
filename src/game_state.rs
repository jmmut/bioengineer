pub mod robots;

use super::map::Map;
use crate::drawing::Drawing;
use crate::gui::GuiActions;
use crate::map::fluids::{FluidMode, Fluids};
use crate::map::transform_cells::Transformation;
use crate::map::CellIndex;
use crate::now;
use crate::game_state::robots::{move_robot_to_position, move_robot_to_tasks, Robot};
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

    fn move_robots_to_position(&mut self, movement_target: &CellIndex) {
        for robot in &mut self.robots {
            let movement_opt = move_robot_to_position(robot.position, &movement_target, &self.map);
            if let Option::Some(movement) = movement_opt {
                robot.position += movement;
                if robot.position == *movement_target {
                    self.movement_queue.pop_front();
                }
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
        for robot in &self.robots {
            let task_opt = self.task_queue.front_mut();
            if let Option::Some(task) = task_opt {
                let transformation_here = task.to_transform.take(&robot.position);
                if transformation_here.is_some() {
                    task.transformation
                        .apply(self.map.get_cell_mut(robot.position));
                    if task.to_transform.len() == 0 {
                        self.task_queue.pop_front();
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
