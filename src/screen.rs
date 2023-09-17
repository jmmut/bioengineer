use crate::screen::input::InputSourceTrait;
use crate::world::World;
use crate::Color;
use drawer_trait::DrawerTrait;
use drawing_state::DrawingState;
use gui::gui_actions::GuiActions;
use gui::Gui;

pub mod assets;
pub mod coords;
pub mod draw_map;
pub mod drawer_trait;
pub mod drawing_state;
pub mod gui;
pub mod hud;
pub mod input;

pub const GREY: Color = Color::new(0.5, 0.5, 0.5, 1.0);

pub struct Screen {
    drawer: Box<dyn DrawerTrait>,
    input_source: Box<dyn InputSourceTrait>,
    gui: Gui,
    drawing_state: DrawingState,
}

impl Screen {
    pub fn new(mut drawer: Box<dyn DrawerTrait>, input_source: Box<dyn InputSourceTrait>) -> Self {
        let gui = Gui::new(drawer.as_mut());
        let drawing_state = DrawingState::new();
        Screen {
            drawer,
            input_source,
            gui,
            drawing_state,
        }
    }

    pub fn get_gui_actions(&mut self, world: &World) -> GuiActions {
        let input = self.input_source.get_input();
        let gui_actions =
            self.gui
                .process_input(input, self.drawer.as_mut(), world, &mut self.drawing_state);
        self.drawing_state.apply_input(&gui_actions);
        gui_actions
    }

    pub fn draw(&mut self, world: &World) {
        draw(self.drawer.as_mut(), world, &self.drawing_state);
    }
}

pub fn draw(drawer: &mut dyn DrawerTrait, world: &World, drawing: &DrawingState) {
    drawer.clear_background(GREY);
    draw_map::draw_map(drawer, world, drawing);
    drawer.ui_draw();
    hud::draw_fps(drawer, &world.game_state);
    hud::draw_level(drawer, drawing.min_cell.y, drawing.max_cell.y);
    hud::draw_networks(drawer, world);
    hud::draw_age(drawer, world);
    hud::draw_life(drawer, world);
}
