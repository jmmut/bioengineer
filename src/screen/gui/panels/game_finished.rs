use crate::screen::drawer_trait::DrawerTrait;
use crate::screen::gui::format_units::format_age;
use crate::screen::gui::{GuiActions, FONT_SIZE};
use crate::screen::main_scene_input::CellSelection;
use crate::world::GameGoalState::{Finished, PostFinished};
use crate::world::World;
use crate::{Rect, Vec2};

pub fn draw_game_finished(
    drawer: &mut dyn DrawerTrait,
    world: &World,
    gui_actions: GuiActions,
) -> GuiActions {
    let mut cell_selection = gui_actions.cell_selection;
    let mut robot_movement = gui_actions.robot_movement;
    let next_game_goal_state = if let Finished(age) = world.goal_state {
        cell_selection = CellSelection::no_selection();
        robot_movement = None;
        let panel_title = "You won!";
        let time_spent = format!("Time spent: {}", format_age(age));
        let text_size_title = drawer.ui_measure_text(panel_title, FONT_SIZE);
        let text_size_age = drawer.ui_measure_text(&time_spent, FONT_SIZE);
        let text_size_x = f32::max(text_size_title.x, text_size_age.x);
        let panel_width = text_size_x + 5.0 * FONT_SIZE;
        let height_per_line = text_size_title.y * 2.0;
        let center = Vec2::new(drawer.screen_width() / 2.0, drawer.screen_height() / 2.0);

        let panel = Rect::new(
            center.x - panel_width / 2.0,
            center.y - height_per_line * 2.0,
            panel_width,
            height_per_line * 7.0,
        );
        let mut new_state = None;
        drawer.ui_named_group(
            panel_title,
            panel.x,
            panel.y,
            panel.w,
            panel.h,
            &mut |drawer| {
                drawer.ui_text(&time_spent);
                if drawer.ui_button("Continue").is_clicked() {
                    new_state = Some(PostFinished)
                }
            },
        );
        new_state

        // TODO: add restarted state
    } else {
        None
    };
    GuiActions {
        robot_movement,
        cell_selection,
        next_game_goal_state,
        ..gui_actions
    }
}
