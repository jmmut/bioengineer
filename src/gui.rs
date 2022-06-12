
use crate::input::Input;
use crate::{DrawingTrait, GameState};
use crate::drawing::hud::show_available_actions;

pub struct Gui;

impl Gui {
    pub fn new() -> Self {
        Gui {}
    }
}

pub struct UnhandledInput {
    pub input: Input,
}

impl Gui {
    pub fn receive_actions(
        self: &mut Self,
        input: Input,
        drawer: &impl DrawingTrait,
        _game_state: &GameState,
    ) -> UnhandledInput {
        show_available_actions(drawer);
        UnhandledInput { input }
    }
}
