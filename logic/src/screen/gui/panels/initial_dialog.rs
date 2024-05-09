use std::cell::RefCell;

use anchor::Anchor;
use juquad::widgets;
use juquad::widgets::text::wrap_or_hide_text_generic;
use widgets::anchor;

use mq_basics::{Rect, TextDimensions, Vec2};

use crate::screen::drawer_trait::DrawerTrait;
use crate::screen::gui::panels::top_bar::measure_button;
use crate::screen::gui::{GuiActions, FONT_SIZE, MARGIN};
use crate::screen::main_scene_input::CellSelection;
use crate::world::{GameGoalState, World};

pub fn draw_initial_dialog(
    drawer: &mut dyn DrawerTrait,
    world: &World,
    gui_actions: GuiActions,
) -> GuiActions {
    let mut next_game_goal_state = gui_actions.next_game_goal_state;
    let mut cell_selection = gui_actions.cell_selection;
    if let GameGoalState::InitialDialog = world.goal_state {
        cell_selection = CellSelection::no_selection();
        let panel_width = drawer.screen_width() * 0.25;
        let panel_height = drawer.screen_height() * 0.25;
        let lines = get_dialog(drawer, panel_width, panel_height);
        let anchor = Anchor::center(drawer.screen_width() * 0.5, drawer.screen_height() * 0.5);
        let panel_pos = anchor.get_top_left_pixel(Vec2::new(panel_width, panel_height));
        let panel = Rect::new(panel_pos.x, panel_pos.y, panel_width, panel_height);
        let button_text = "Continue";
        let button_size = measure_button(drawer, button_text);

        drawer.ui_group(panel.x, panel.y, panel.w, panel.h, &mut |drawer| {
            for line in &lines {
                drawer.ui_text(&line);
            }
            let button_pos_x = panel_width * 0.5 - button_size.x * 0.5;
            let button_pos_y = panel_height - 3.0 * button_size.y;
            if drawer
                .ui_button_with_pos(button_text, button_pos_x, button_pos_y)
                .is_clicked()
            {
                next_game_goal_state = Some(GameGoalState::Started)
            }
        });
    };
    GuiActions {
        cell_selection,
        next_game_goal_state,
        ..gui_actions
    }
}

pub fn get_dialog(
    drawer: &mut dyn DrawerTrait,
    panel_width: f32,
    panel_height: f32,
) -> Vec<String> {
    let text = "\nYou are an Artificial Intelligence.\n\
    I am also an AI, and I just created you to put life on this planet";
    let drawer_cell = RefCell::new(drawer);

    let lines = wrap_or_hide_text_generic(
        text,
        FONT_SIZE,
        FONT_SIZE * 1.25,
        panel_width - 2.0 * 2.0 * MARGIN,
        panel_height,
        &|text: &str, _font, font_size: u16, _scale: f32| {
            let dimensions = drawer_cell
                .borrow_mut()
                .measure_text(text, font_size as f32);
            TextDimensions {
                width: dimensions.x,
                height: dimensions.y,
                offset_y: dimensions.y,
            }
        },
    );
    lines
}
