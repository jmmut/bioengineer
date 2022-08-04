use drawing::Drawing;
use gui_actions::GuiActions;
use crate::world::World;
use crate::{draw, DrawerTrait, Gui, InputSourceTrait};

pub mod drawing;
pub mod gui;
pub mod input;
pub mod gui_actions;

pub struct Screen<Drawer: DrawerTrait, InputSource: InputSourceTrait> {
    drawer: Drawer,
    input_source: InputSource,
    gui: Gui,
    drawing_state: Drawing,
}

impl<Drawer: DrawerTrait, InputSource: InputSourceTrait> Screen<Drawer, InputSource> {
    pub fn new(mut drawer: Drawer, input_source: InputSource) -> Self {
        let gui = gui::Gui::new(&mut drawer);
        let drawing_state = Drawing::new();
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
                .receive_actions(input, &self.drawer, &world.game_state, &self.drawing_state);
        self.drawing_state
            .apply_input(&gui_actions, self.drawer.screen_width());
        gui_actions
    }

    pub fn draw(&self, world: &World) {
        draw(&self.drawer, &world.game_state, &self.drawing_state);
    }
}
