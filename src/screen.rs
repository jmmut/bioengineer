use crate::gui_actions::GuiActions;
use crate::world::World;
use crate::{DrawerTrait, InputSourceTrait};

pub struct Screen<Drawer: DrawerTrait, InputSource: InputSourceTrait> {
    drawer: Drawer,
    input_source: InputSource,
}

impl<Drawer: DrawerTrait, InputSource: InputSourceTrait> Screen<Drawer, InputSource> {
    pub fn new(drawer: Drawer, input_source: InputSource) -> Self {
        Screen {
            drawer,
            input_source,
        }
    }

    pub fn get_gui_actions(&mut self) -> GuiActions {
        todo!()
    }

    pub fn draw(&self, world: &World) {
        todo!()
    }
}
