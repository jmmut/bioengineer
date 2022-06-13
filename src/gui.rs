use crate::drawing::hud;
use crate::input::Input;
use crate::map::mechanics::Transformation;
use crate::Color;
use crate::{DrawingTrait, GameState};

pub struct Gui;

pub const FONT_SIZE: f32 = 20.0;
pub const BLACK: Color = Color::new(0.0, 0.0, 0.0, 1.0);
pub const TEXT_COLOR: Color = BLACK;
pub const BACKGROUND_UI_COLOR: Color = Color::new(0.3, 0.3, 0.4, 1.0);
pub const BACKGROUND_UI_COLOR_BUTTON: Color = Color::new(0.32, 0.32, 0.42, 1.0);
pub const BACKGROUND_UI_COLOR_HOVERED: Color = Color::new(0.35, 0.35, 0.45, 1.0);
pub const BACKGROUND_UI_COLOR_CLICKED: Color = Color::new(0.25, 0.25, 0.35, 1.0);

impl Gui {
    pub fn new(drawer: &mut impl DrawingTrait) -> Self {
        Self::set_skin(drawer);
        Gui {}
    }
}

pub struct GuiActions {
    pub input: Input,
    pub selected_cell_transformation: Option<Transformation>,
}

impl Gui {
    pub fn receive_actions(
        self: &mut Self,
        input: Input,
        drawer: &impl DrawingTrait,
        _game_state: &GameState,
    ) -> GuiActions {
        let unhandled_input = hud::show_available_actions(drawer, _game_state, input);
        unhandled_input
    }
    fn set_skin(drawer: &mut impl DrawingTrait) {
        drawer.set_button_style(
            FONT_SIZE,
            TEXT_COLOR,
            BACKGROUND_UI_COLOR_BUTTON,
            BACKGROUND_UI_COLOR_HOVERED,
            BACKGROUND_UI_COLOR_CLICKED,
        );
    }
}
