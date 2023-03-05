use crate::screen::drawer_trait::{DrawerTrait, Interaction};
use crate::screen::drawing_state::{DrawingState, TopBarShowing};
use crate::screen::gui::{GuiActions, FONT_SIZE, MARGIN};
use crate::world::game_state::{get_goal_air_cleaned, get_goal_air_cleaned_str};
use crate::Vec2;
pub const TOP_BAR_HEIGHT: f32 = FONT_SIZE * 3.0;

pub fn draw_top_bar(
    drawer: &impl DrawerTrait,
    drawing: &mut DrawingState,
    gui_actions: GuiActions,
) -> GuiActions {
    let panel_height = TOP_BAR_HEIGHT;
    let mut goals = Interaction::None;
    let mut help = Interaction::None;
    drawer.ui_group(0.0, 0.0, drawer.screen_width(), panel_height, || {
        goals = drawer.ui_button("Goals");
        drawer.ui_same_line();
        help = drawer.ui_button("Help");
    });
    maybe_draw_goals(drawer, drawing, goals);
    maybe_draw_help(drawer, drawing, help);
    gui_actions
}

fn maybe_draw_goals(drawer: &impl DrawerTrait, drawing: &mut DrawingState, goals: Interaction) {
    maybe_draw_pop_up(
        drawer,
        drawing,
        "Goals",
        goals,
        TopBarShowing::Goals,
        goals_text_lines(),
    );
}

fn maybe_draw_pop_up(
    drawer: &impl DrawerTrait,
    drawing: &mut DrawingState,
    pop_up_name: &str,
    pop_up_click: Interaction,
    showing: TopBarShowing,
    text: Vec<String>,
) {
    if pop_up_click.is_clicked() {
        toggle_showing_or_none(&mut drawing.top_bar_showing, showing.clone());
    }
    if drawing.top_bar_showing == showing {
        let center = Vec2::new(drawer.screen_width() / 2.0, drawer.screen_height() / 2.0);
        let panel_size = Vec2::new(550.0, 300.0);
        let button_text = "Continue".to_string();
        let button_size = drawer.measure_text(&button_text, FONT_SIZE);
        // let button_size = Vec2::new(button_size.x / button_text.len() as f32 * (button_text.len() + 6) as f32, button_size.y * 2.0);
        let title_height = FONT_SIZE * 3.0;
        let button_size = Vec2::new(button_size.x + MARGIN * 4.0, button_size.y + MARGIN);
        drawer.ui_named_group(
            pop_up_name,
            center.x - panel_size.x / 2.0,
            center.y - panel_size.y / 2.0,
            panel_size.x,
            panel_size.y,
            || {
                for line in text {
                    drawer.ui_text(&line);
                }
                let button_pos_x = panel_size.x / 2.0 - button_size.x / 2.0;
                let button_pos_y = panel_size.y - title_height - button_size.y * 2.0;
                if drawer
                    .ui_button_with_pos(&button_text, button_pos_x, button_pos_y)
                    .is_clicked()
                {
                    drawing.top_bar_showing = TopBarShowing::None;
                }
            },
        );
    }
}

fn toggle_showing_or_none(top_bar_showing: &mut TopBarShowing, showing: TopBarShowing) {
    *top_bar_showing = if *top_bar_showing == showing {
        TopBarShowing::None
    } else {
        showing
    };
}

fn goals_text_lines() -> Vec<String> {
    vec![
        "You are an Artificial Intelligence sent to this barren planet".to_string(),
        "to put life on it.".to_string(),
        "".to_string(),
        "You have to:".to_string(),
        format!(
            "- Clean {} of air ({} liters)",
            get_goal_air_cleaned_str(),
            get_goal_air_cleaned()
        ),
    ]
}

fn maybe_draw_help(drawer: &impl DrawerTrait, drawing: &mut DrawingState, help: Interaction) {
    maybe_draw_pop_up(
        drawer,
        drawing,
        "Help",
        help,
        TopBarShowing::Help,
        help_text_lines(),
    );
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
- m: reset map (delete all constructions)"#.to_string();
    text.split("\n").map(|s| s.to_string()).collect()
}
