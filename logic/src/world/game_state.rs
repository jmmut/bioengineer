use crate::screen::gui::format_units::format_unit;
use crate::screen::gui::GuiActions;
use mq_basics::now;

pub const DEFAULT_PROFILE_ENABLED: bool = false;
pub const DEFAULT_ADVANCING_FLUIDS: bool = false;
const DEFAULT_ADVANCE_FLUID_EVERY_N_FRAMES: i32 = 10;
const DEFAULT_ADVANCE_ROBOTS_EVERY_N_FRAMES: i32 = 15;
const DEFAULT_AGING_EVERY_N_FRAMES: i32 = 15;

enum Phase {
    Robots = 0,
    Fluids = 1,
    Aging = 2,
}

pub struct GameState {
    pub frame_index: i32,
    pub previous_frame_ts: f64,
    pub current_frame_ts: f64,
    pub profile: bool,
    advancing_fluids: bool,
    advancing_fluids_single_step: bool,
    advance_fluid_every_n_frames: i32,
    advance_robots_every_n_frames: i32,
    age_every_n_frames: i32,
}

impl GameState {
    pub fn new(fluids: bool) -> GameState {
        GameState {
            frame_index: 0,
            previous_frame_ts: now() - 1.0,
            current_frame_ts: now(),
            advancing_fluids: fluids,
            advancing_fluids_single_step: false,
            advance_fluid_every_n_frames: DEFAULT_ADVANCE_FLUID_EVERY_N_FRAMES,
            advance_robots_every_n_frames: DEFAULT_ADVANCE_ROBOTS_EVERY_N_FRAMES,
            profile: DEFAULT_PROFILE_ENABLED,
            age_every_n_frames: DEFAULT_AGING_EVERY_N_FRAMES,
        }
    }

    pub fn update_with_gui_actions(&mut self, gui_actions: &GuiActions) {
        if gui_actions.toggle_fluids {
            self.advancing_fluids = !self.advancing_fluids;
        }

        self.advancing_fluids_single_step = gui_actions.single_fluid;
    }

    pub fn should_advance_robots_this_frame(&self) -> bool {
        let should_process_frame = (self.frame_index + self.advance_robots_every_n_frames
            - Phase::Robots as i32)
            % self.advance_robots_every_n_frames
            == 0;
        should_process_frame
    }

    pub fn should_advance_fluids_this_frame(&self) -> bool {
        if self.advancing_fluids_single_step {
            return true;
        } else {
            if self.advancing_fluids {
                let should_process_frame = (self.frame_index + self.advance_fluid_every_n_frames
                    - Phase::Fluids as i32)
                    % self.advance_fluid_every_n_frames
                    == 0;
                return should_process_frame;
            }
        }
        return false;
    }

    pub fn should_age_this_frame(&self) -> bool {
        let should_age = (self.frame_index + self.age_every_n_frames - Phase::Aging as i32)
            % self.age_every_n_frames
            == 0;
        return should_age;
    }

    pub fn set_advance_every_frame(&mut self) {
        self.advance_robots_every_n_frames = 1;
        self.advance_fluid_every_n_frames = 1;
        self.age_every_n_frames = 1;
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_advance_robots() {
        let mut game_state = GameState::new(true);
        assert_eq!(game_state.should_advance_robots_this_frame(), true);
        game_state.advance_frame();
        for i in 1..DEFAULT_ADVANCE_ROBOTS_EVERY_N_FRAMES {
            assert_eq!(
                game_state.should_advance_robots_this_frame(),
                false,
                "on iteration {}",
                i
            );
            game_state.advance_frame();
        }
        assert_eq!(game_state.should_advance_robots_this_frame(), true);
    }

    #[test]
    fn test_advance_robots_every_frame() {
        let mut game_state = GameState::new(true);
        game_state.set_advance_every_frame();
        assert_eq!(game_state.should_advance_robots_this_frame(), true);
        game_state.advance_frame();
        assert_eq!(game_state.should_advance_robots_this_frame(), true);
        game_state.advance_frame();
        assert_eq!(game_state.should_advance_robots_this_frame(), true);
    }

    #[test]
    fn test_advance_fluids() {
        let mut game_state = GameState::new(true);
        assert_eq!(game_state.should_advance_fluids_this_frame(), false);
        game_state.advance_frame();
        assert_eq!(game_state.should_advance_fluids_this_frame(), true);
        game_state.advance_frame();
        for i in 1..DEFAULT_ADVANCE_FLUID_EVERY_N_FRAMES {
            assert_eq!(
                game_state.should_advance_fluids_this_frame(),
                false,
                "on iteration {}",
                i
            );
            game_state.advance_frame();
        }
        assert_eq!(game_state.should_advance_fluids_this_frame(), true);
    }

    #[test]
    fn test_advance_fluids_every_frame() {
        let mut game_state = GameState::new(true);
        game_state.set_advance_every_frame();
        assert_eq!(game_state.should_advance_fluids_this_frame(), true);
        game_state.advance_frame();
        assert_eq!(game_state.should_advance_fluids_this_frame(), true);
        game_state.advance_frame();
        assert_eq!(game_state.should_advance_fluids_this_frame(), true);
    }

    #[test]
    fn test_age() {
        let mut game_state = GameState::new(true);
        assert_eq!(game_state.should_age_this_frame(), false);
        game_state.advance_frame();
        assert_eq!(game_state.should_age_this_frame(), false);
        game_state.advance_frame();
        assert_eq!(game_state.should_age_this_frame(), true);
        game_state.advance_frame();
        for i in 1..DEFAULT_AGING_EVERY_N_FRAMES {
            assert_eq!(
                game_state.should_age_this_frame(),
                false,
                "on iteration {}",
                i
            );
            game_state.advance_frame();
        }
        assert_eq!(game_state.should_age_this_frame(), true);
    }

    #[test]
    fn test_age_every_frame() {
        let mut game_state = GameState::new(true);
        game_state.set_advance_every_frame();

        assert_eq!(game_state.should_age_this_frame(), true);
        game_state.advance_frame();
        assert_eq!(game_state.should_age_this_frame(), true);
        game_state.advance_frame();
        assert_eq!(game_state.should_age_this_frame(), true);
    }
}
