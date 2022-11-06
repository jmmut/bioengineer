use crate::now;
use crate::screen::gui::GuiActions;
use crate::world::networks::network::format_unit;

pub const DEFAULT_PROFILE_ENABLED: bool = false;
const DEFAULT_ADVANCING_FLUIDS: bool = true;
const DEFAULT_ADVANCE_FLUID_EVERY_N_FRAMES: i32 = 10;
const DEFAULT_ADVANCE_ROBOTS_EVERY_N_FRAMES: i32 = 15;

pub struct GameState {
    pub frame_index: i32,
    pub previous_frame_ts: f64,
    pub current_frame_ts: f64,
    pub profile: bool,
    advancing_fluids: bool,
    advancing_fluids_single_step: bool,
    advance_fluid_every_n_frames: i32,
    advance_robots_every_n_frames: i32,
}

impl GameState {
    pub fn new() -> GameState {
        GameState {
            frame_index: 0,
            previous_frame_ts: now() - 1.0,
            current_frame_ts: now(),
            advancing_fluids: DEFAULT_ADVANCING_FLUIDS,
            advancing_fluids_single_step: false,
            advance_fluid_every_n_frames: DEFAULT_ADVANCE_FLUID_EVERY_N_FRAMES,
            advance_robots_every_n_frames: DEFAULT_ADVANCE_ROBOTS_EVERY_N_FRAMES,
            profile: DEFAULT_PROFILE_ENABLED,
        }
    }

    pub fn update_with_gui_actions(&mut self, gui_actions: &GuiActions) {
        if gui_actions.input.toggle_fluids {
            self.advancing_fluids = !self.advancing_fluids;
        }

        self.advancing_fluids_single_step = gui_actions.input.single_fluid;
    }

    pub fn should_advance_robots_this_frame(&mut self) -> bool {
        let should_process_frame = self.frame_index % self.advance_robots_every_n_frames == 0;
        should_process_frame
    }

    pub fn should_advance_fluids_this_frame(&mut self) -> bool {
        if self.advancing_fluids_single_step {
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
        self.frame_index = (self.frame_index + 1) % 3600;
        self.previous_frame_ts = self.current_frame_ts;
        self.current_frame_ts = now();
    }
}

pub fn get_goal_air_cleaned() -> f64 {
    100_000.0
}

pub fn get_goal_air_cleaned_str() -> String {
    format_unit(get_goal_air_cleaned(), "L")
}
