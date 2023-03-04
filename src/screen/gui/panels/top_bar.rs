use macroquad::logging::info;
use crate::screen::drawer_trait::{DrawerTrait, Interaction};
use crate::screen::gui::{FONT_SIZE, GuiActions};
use crate::world::World;
use crate::{Rect, Vec2};
use crate::screen::drawing_state::{DrawingState, TopBarShowing};
use crate::screen::gui::units::format_unit;
use crate::world::game_state::{get_goal_air_cleaned, get_goal_air_cleaned_str};
pub const TOP_BAR_HEIGHT: f32 = FONT_SIZE * 3.0;

pub fn draw_top_bar(
    drawer: &impl DrawerTrait,
    drawing: &mut DrawingState,
    gui_actions: GuiActions,
) -> GuiActions {
    let panel_height = TOP_BAR_HEIGHT;
    let mut goals = Interaction::None;
    let mut help = Interaction::None;
    let panel = drawer.ui_group(0.0, 0.0, drawer.screen_width(), panel_height, || {
        goals = drawer.ui_button("Goals");
        drawer.ui_same_line();
        help = drawer.ui_button("Help");
    });
    maybe_draw_goals(drawer, drawing, goals);
    maybe_draw_help(drawer, drawing, help);
    gui_actions
}

fn maybe_draw_goals(drawer: &impl DrawerTrait, drawing: &mut DrawingState, goals: Interaction) {
    if goals.is_clicked() {
        drawing.top_bar_showing = TopBarShowing::Goals;
    }
    if drawing.top_bar_showing == TopBarShowing::Goals {
        drawer.ui_named_group(
            "Goals",
            drawer.screen_width() / 3.0, drawer.screen_height() / 3.0,
            drawer.screen_width() / 3.0, drawer.screen_height() / 3.0,
            || {
                for line in goals_text_lines() {
                    drawer.ui_text(&line);
                }
                if drawer.ui_button("Continue").is_clicked() {
                    drawing.top_bar_showing = TopBarShowing::None;
                }
            });
    }
}

fn goals_text_lines() -> Vec<String> {
    vec![
        "You are an Artificial Intelligence sent to this barren planet".to_string(),
        "to put life on it.".to_string(),
        "".to_string(),
        "You have to:".to_string(),
        format!("- Clean {} of air ({} liters)", get_goal_air_cleaned_str(), get_goal_air_cleaned()),
        "".to_string(),
        "".to_string(),
        "".to_string(),
        "".to_string(),
        "".to_string(),
        "".to_string(),
    ]
}

fn maybe_draw_help(drawer: &impl DrawerTrait, drawing: &mut DrawingState, help: Interaction) {
    if help.is_clicked() {
        drawing.top_bar_showing = TopBarShowing::Help;
    }
    if drawing.top_bar_showing == TopBarShowing::Help {
        drawer.ui_named_group(
            "Help",
            drawer.screen_width() / 3.0, drawer.screen_height() / 3.0,
            drawer.screen_width() / 3.0, drawer.screen_height() / 3.0,
            || {
                for line in help_text_lines() {
                    drawer.ui_text(&line);
                }
                if drawer.ui_button("Continue").is_clicked() {
                    drawing.top_bar_showing = TopBarShowing::None;
                }
            });
    }
}

fn help_text_lines() -> Vec<String> {
    let text = r#"Controls
- click (optionally drag): select cells
  - CTRL + left click (opt. drag): add cells to the selection
  - CTRL + right click (opt. drag): remove cells from the selection
- right click: move the robot
- arrow UP and DOWN, mouse wheel up and down: change height layer
- mouse wheel click and drag: move the map horizontally
- r: reset timer and accumulated production
- m: reset map (delete all constructions)

"#.to_string();
    text.split("\n").map(|s| {s.to_string()}).collect()
}
