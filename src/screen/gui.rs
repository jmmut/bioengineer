pub mod format_units;
pub mod gui_actions;
mod panels;

pub use gui_actions::GuiActions;

use crate::screen::coords::cell_pixel::{clicked_cell, pixel_to_subcell_offset};
use crate::screen::drawer_trait::DrawerTrait;
use crate::screen::drawing_state::DrawingState;
use crate::screen::gui::panels::top_bar::draw_top_bar;
use crate::screen::gui::panels::{
    draw_available_transformations::show_available_transformations,
    game_finished::draw_game_finished, task_queue::draw_robot_queue,
};
use crate::screen::input::{CellIndexSelection, CellSelection, Input, PixelCellSelection, PixelPosition, ZoomChange};
use crate::world::map::CellIndex;
use crate::{Color, World};

pub const FONT_SIZE: f32 = 16.0;
pub const MARGIN: f32 = 10.0;
pub const BLACK: Color = Color::new(0.0, 0.0, 0.0, 1.0);
pub const TEXT_COLOR: Color = BLACK;
pub const BUTTON_TEXT_COLOR: Color = BLACK;
pub const TEXT_COLOR_ALARM: Color = Color::new(0.40, 0.0, 0.0, 1.00);
pub const BACKGROUND_UI_COLOR: Color = Color::new(0.7, 0.7, 0.9, 1.0);
pub const BACKGROUND_UI_COLOR_BUTTON: Color = Color::new(0.85, 0.85, 1.0, 1.0);
pub const BACKGROUND_UI_COLOR_BUTTON_HOVERED: Color = Color::new(0.8, 0.9, 0.7, 1.0);
pub const BACKGROUND_UI_COLOR_BUTTON_CLICKED: Color = Color::new(0.8, 0.8, 0.9, 1.0);

pub struct Gui;

impl Gui {
    pub fn new(drawer: &mut impl DrawerTrait) -> Self {
        Self::set_skin(drawer);
        Gui {}
    }
}

impl Gui {
    pub fn process_input(
        &self,
        input: Input,
        drawer: &impl DrawerTrait,
        world: &World,
        drawing: &mut DrawingState, // TODO: make const by add top_bar_showing to GuiActions
    ) -> GuiActions {
        let unhandled_input = GuiActions {
            // input: input.clone(),
            cell_selection: pixel_to_cell_selection(input.cell_selection, drawer, drawing),
            selected_cell_transformation: Option::None,
            robot_movement: robot_movement_pixel_to_cell(input.robot_movement, drawer, drawing),
            go_to_robot: Option::None,
            cancel_task: Option::None,
            do_now_task: Option::None,
            next_game_goal_state: Option::None,
            regenerate_map: input.regenerate_map,
            toggle_profiling: input.toggle_profiling,
            toggle_fluids: input.toggle_fluids,
            single_fluid: input.single_fluid,
            reset_quantities: input.reset_quantities,
            quit: input.quit,
            change_height_rel: input.change_height_rel,
            move_map_horizontally_diff: pixel_to_subcell_offset(
                input.move_map_horizontally,
                drawing.zoom,
            ),
            zoom_change: input.zoom_change,
        };
        let unhandled_input = draw_game_finished(drawer, world, unhandled_input);
        let unhandled_input =
            show_available_transformations(drawer, world, unhandled_input, drawing);

        let unhandled_input = draw_robot_queue(drawer, world, unhandled_input);
        let unhandled_input = draw_top_bar(drawer, drawing, unhandled_input);
        unhandled_input
    }
    fn set_skin(drawer: &mut impl DrawerTrait) {
        drawer.set_style(
            FONT_SIZE,
            TEXT_COLOR,
            BUTTON_TEXT_COLOR,
            BACKGROUND_UI_COLOR,
            BACKGROUND_UI_COLOR_BUTTON,
            BACKGROUND_UI_COLOR_BUTTON_HOVERED,
            BACKGROUND_UI_COLOR_BUTTON_CLICKED,
        );
    }
}

fn pixel_to_cell_selection(
    pixel_selection: PixelCellSelection,
    drawer: &impl DrawerTrait,
    drawing: &DrawingState,
) -> CellSelection {
    CellSelection {
        state: pixel_selection.state,
        selection: pixel_selection
            .pixel_selection
            .map(|selection| CellIndexSelection {
                start: clicked_cell(selection.start, drawer.screen_width(), drawing),
                end: clicked_cell(selection.end, drawer.screen_width(), drawing),
            }),
        selection_type: pixel_selection.selection_type,
    }
}

fn robot_movement_pixel_to_cell(
    robot_movement: Option<PixelPosition>,
    drawer: &impl DrawerTrait,
    drawing: &DrawingState,
) -> Option<CellIndex> {
    robot_movement.map(|click| clicked_cell(click, drawer.screen_width(), drawing))
}
