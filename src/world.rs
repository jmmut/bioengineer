
pub mod map;
pub mod game_state;

use crate::screen::gui_actions::GuiActions;
use crate::GameState;

pub struct World {
    pub game_state: GameState,
}

impl World {
    pub fn new() -> Self {
        World {
            game_state: GameState::new(),
        }
    }

    /// returns if the game should do another iteration
    pub fn update(&mut self, gui_actions: GuiActions) -> bool {
        self.game_state.update_with_gui_actions(&gui_actions);
        self.game_state.advance_frame();
        gui_actions.should_continue()
    }
}
