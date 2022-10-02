use crate::now;
use crate::screen::gui::GuiActions;
use crate::world::game_state::GameGoalState::Started;
use crate::world::map::fluids::{FluidMode, Fluids};
use crate::world::map::transform_cells::Transformation;
use crate::world::map::CellIndex;
use crate::world::map::Map;
use crate::world::networks::{format_unit, Networks};
use crate::world::robots::{
    is_position_actionable, move_robot_to_position, move_robot_to_tasks, reachable_positions, Robot,
};
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
    pub advancing_fluids: bool,
    pub advance_fluid_every_n_frames: i32,
    pub advance_robots_every_n_frames: i32,
    pub profile: bool, // TODO: move to DrawingState?
}

impl GameState {
    pub fn new() -> GameState {
        GameState {
            frame_index: 0,
            previous_frame_ts: now() - 1.0,
            current_frame_ts: now(),
            advancing_fluids: DEFAULT_ADVANCING_FLUIDS,
            advance_fluid_every_n_frames: DEFAULT_ADVANCE_FLUID_EVERY_N_FRAMES,
            advance_robots_every_n_frames: DEFAULT_ADVANCE_ROBOTS_EVERY_N_FRAMES,
            profile: DEFAULT_PROFILE_ENABLED,
        }
    }

    pub fn update_with_gui_actions(&mut self, gui_actions: GuiActions) {
        if gui_actions.input.toggle_profiling {
            self.profile = !self.profile;
        }
        if gui_actions.input.toggle_fluids {
            self.advancing_fluids = !self.advancing_fluids;
        }
    }

    pub(crate) fn should_advance_robots_this_frame(&mut self) -> bool {
        let should_process_frame = self.frame_index % self.advance_robots_every_n_frames == 0;
        should_process_frame
    }

    pub fn should_advance_fluids_this_frame(&mut self, gui_actions: &GuiActions) -> bool {
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

pub fn get_goal_air_cleaned() -> f64 {
    100000.0
}

pub fn get_goal_air_cleaned_str() -> String {
    format_unit(100000.0, "L")
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
