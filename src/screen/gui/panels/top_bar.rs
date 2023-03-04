use crate::screen::drawer_trait::DrawerTrait;
use crate::screen::gui::{FONT_SIZE, GuiActions};
use crate::world::World;

pub const TOP_BAR_HEIGHT: f32 = FONT_SIZE * 3.0;

pub fn draw_top_bar(
    drawer: &impl DrawerTrait,
    world: &World,
    gui_actions: GuiActions,
) -> GuiActions {
    let panel_height = TOP_BAR_HEIGHT;
    let panel = drawer.ui_group(0.0, 0.0, drawer.screen_width(), panel_height, || {

    });
    gui_actions
}
