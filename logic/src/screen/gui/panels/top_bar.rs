use crate::screen::drawer_trait::{DrawerTrait, Interaction};
use crate::screen::drawing_state::{DrawingState, TopBarShowing};
use crate::screen::gui::format_units::format_liters;
use crate::screen::gui::{GuiActions, FONT_SIZE, MARGIN};
use crate::screen::main_scene_input::{CellSelection, ZoomChange};
use crate::world::game_state::{get_goal_air_cleaned, get_goal_air_cleaned_str};
use crate::world::{World, LIFE_COUNT_REQUIRED_FOR_WINNING};
use mq_basics::Vec2;

pub const TOP_BAR_HEIGHT: f32 = FONT_SIZE * 3.0;

pub fn draw_top_bar(
    drawer: &mut dyn DrawerTrait,
    world: &World,
    drawing: &mut DrawingState,
    gui_actions: GuiActions,
) -> GuiActions {
    let panel_height = TOP_BAR_HEIGHT;
    let mut goals = Interaction::None;
    let mut help = Interaction::None;
    let mut cell_selection = gui_actions.cell_selection;

    let mut interactions = Vec::new();
    interactions.push(drawer.ui_group(
        0.0,
        0.0,
        drawer.screen_width(),
        panel_height,
        &mut |drawer| {
            drawer.ui_same_line(&mut |drawer: &mut dyn DrawerTrait| {
                goals = drawer.ui_button("Goals");
                help = drawer.ui_button("Help");
                drawer.ui_text(&format!(
                    "    Render depth: {}",
                    drawing.max_cell.y - drawing.min_cell.y
                ));
                if drawer.ui_button("+").is_clicked() {
                    drawing.min_cell.y = (drawing.min_cell.y - 1).max(drawing.max_cell.y - 15);
                    drawing.maybe_change_height_rel(0, None);
                }
                if drawer.ui_button("-").is_clicked() {
                    drawing.min_cell.y = (drawing.min_cell.y + 1).min(drawing.max_cell.y - 2);
                    drawing.maybe_change_height_rel(0, None);
                }
                drawer.ui_text(&format!("    Zoom: {:.0}%", (drawing.zoom * 100.0).round()));
                if drawer.ui_button("+").is_clicked() {
                    drawing.update_zoom(ZoomChange::ZoomIn)
                }
                if drawer.ui_button("-").is_clicked() {
                    drawing.update_zoom(ZoomChange::ZoomOut)
                }
            });
        },
    ));
    interactions.push(maybe_draw_goals(drawer, drawing, world, goals));
    interactions.push(maybe_draw_help(drawer, drawing, help));

    for interaction in &interactions {
        if interaction.is_hovered_or_clicked() {
            cell_selection = CellSelection::no_selection();
        }
    }
    if cell_selection.is_something_being_selected() {
        drawing.top_bar_showing = TopBarShowing::None
    }

    GuiActions {
        cell_selection,
        ..gui_actions
    }
}

fn maybe_draw_goals(
    drawer: &mut dyn DrawerTrait,
    drawing: &mut DrawingState,
    world: &World,
    goals: Interaction,
) -> Interaction {
    if goals.is_clicked() {
        toggle_showing_or_none(&mut drawing.top_bar_showing, TopBarShowing::Goals);
    }
    return if drawing.top_bar_showing == TopBarShowing::Goals {
        let text_lines = goals_text_lines(
            world.networks.get_total_air_cleaned(),
            world.networks.get_non_ship_machine_count(),
            world.life.len(),
        );
        draw_pop_up(drawer, drawing, "Goals", &text_lines, |_| {})
    } else {
        Interaction::None
    };
}

pub fn draw_pop_up<F: FnMut(&mut dyn DrawerTrait) -> ()>(
    drawer: &mut dyn DrawerTrait,
    drawing: &mut DrawingState,
    pop_up_name: &str,
    text: &Vec<String>,
    mut draw_extra_widgets: F,
) -> Interaction {
    let center = Vec2::new(drawer.screen_width() * 0.5, drawer.screen_height() * 0.5);
    let title_height = FONT_SIZE * 2.0;
    let button_text = "Continue";
    let button_size = measure_button(drawer, button_text);
    let text_size = measure_text(drawer, &text);
    let panel_size = Vec2::new(
        text_size.x + MARGIN * 4.0,
        title_height + text_size.y + button_size.y + MARGIN * 5.0,
    );
    drawer.ui_named_group(
        pop_up_name,
        center.x - panel_size.x * 0.5,
        center.y - panel_size.y * 0.5,
        panel_size.x,
        panel_size.y,
        &mut |drawer| {
            for line in text {
                drawer.ui_text(&line);
            }
            draw_extra_widgets(drawer);
            let button_pos_x = panel_size.x * 0.5 - button_size.x * 0.5;
            let button_pos_y = title_height + text_size.y;
            if drawer
                .ui_button_with_pos(button_text, button_pos_x, button_pos_y)
                .is_clicked()
            {
                drawing.top_bar_showing = TopBarShowing::None;
            }
        },
    )
}

fn measure_text(drawer: &mut dyn DrawerTrait, text: &Vec<String>) -> Vec2 {
    let text_height = text.len() as f32 * FONT_SIZE * 1.2;
    let text_width = measure_longest_width(drawer, &text);
    Vec2::new(text_width, text_height)
}

fn measure_longest_width(drawer: &mut dyn DrawerTrait, text: &Vec<String>) -> f32 {
    let mut max_width = 0.0;
    for line in text {
        let line_width = drawer.ui_measure_text(line, FONT_SIZE).x;
        if line_width > max_width {
            max_width = line_width;
        }
    }
    max_width
}

pub fn measure_button(drawer: &mut dyn DrawerTrait, button_text: &str) -> Vec2 {
    let button_size = drawer.ui_measure_text(&button_text, FONT_SIZE);
    // let button_size = Vec2::new(button_size.x / button_text.len() as f32 * (button_text.len() + 6) as f32, button_size.y * 2.0);
    let button_size = Vec2::new(button_size.x + MARGIN * 4.0, button_size.y + MARGIN);
    button_size
}

fn toggle_showing_or_none(top_bar_showing: &mut TopBarShowing, showing: TopBarShowing) {
    *top_bar_showing = if *top_bar_showing == showing {
        TopBarShowing::None
    } else {
        showing
    };
}

fn goals_text_lines(air_cleaned: f64, machines: i32, trees: usize) -> Vec<String> {
    fn get_symbol_is_done(done: bool) -> &'static str {
        if done {
            ""
        } else {
            ""
        }
    }
    let goal_air = get_goal_air_cleaned();
    let air_cleaned_str = format_liters(air_cleaned);
    let goal_air_str = get_goal_air_cleaned_str();
    let air_done = get_symbol_is_done(air_cleaned >= get_goal_air_cleaned());
    let machines_done = get_symbol_is_done(machines == 0);
    let trees_goal = LIFE_COUNT_REQUIRED_FOR_WINNING;
    let trees_done = get_symbol_is_done(trees == LIFE_COUNT_REQUIRED_FOR_WINNING);
    let progress_bar_air = progress_bar(air_cleaned, goal_air);
    let progress_bar_machines = progress_bar((4.0 - (machines as f64 + 1.0).log10()).max(0.0), 4.0);
    let progress_bar_trees = progress_bar(trees as f64, trees_goal as f64);

    // TODO: draw progress bars
    format!(
        r#"You are an Artificial Intelligence sent to this barren planet
to put life on it.

You have to:
{progress_bar_air} Clean {goal_air} liters of air (or more): {air_cleaned_str}/{goal_air_str} {air_done}
{progress_bar_machines} Have no machines: {machines}/0 {machines_done}
{progress_bar_trees} Keep {trees_goal} trees alive (or more): {trees}/{trees_goal} {trees_done}"#,
    )
    .split("\n")
    .map(|s| s.to_string())
    .collect()
}

fn progress_bar(progress: f64, goal: f64) -> &'static str {
    let ratio = progress / goal;
    if ratio < 0.25 {
        "...."
    } else if ratio < 0.5 {
        "|..."
    } else if ratio < 0.75 {
        "||.."
    } else if ratio < 1.0 {
        "|||."
    } else {
        "||||"
    }
}

fn maybe_draw_help(
    drawer: &mut dyn DrawerTrait,
    drawing: &mut DrawingState,
    help: Interaction,
) -> Interaction {
    if help.is_clicked() {
        toggle_showing_or_none(&mut drawing.top_bar_showing, TopBarShowing::Help);
    }
    return if drawing.top_bar_showing == TopBarShowing::Help {
        draw_pop_up(drawer, drawing, "Help", &help_text_lines(), |_| {})
    } else {
        Interaction::None
    };
}

fn help_text_lines() -> Vec<String> {
    let text = r#"Controls
- click (optionally drag): select cells
  - CTRL (or CMD) + left click (opt. drag): add cells to the selection
  - CTRL (or CMD) + right click (opt. drag): remove cells from the selection
- arrow UP and DOWN, or W and S, or mouse wheel up and down: change height layer
- mouse wheel click and drag: move the map horizontally
- Q and D: move the map horizontally to top left or to bottom down (Z axis in 3D)
- A and E: move the map horizontally to bottom left or to top right (X axis 3D)
- SHIFT + {W,A,S,D,Q,E}: faster move
- G: go to spaceship
- CTRL (or CMD) + wheel: zoom in or out
- L: enable or disable fluid simulaton (CPU heavy)
- N: single step of fluid simulation
- R: reset timer and accumulated production
- M: regenerate map (delete all constructions)"#
        .to_string();
    text.split("\n").map(|s| s.to_string()).collect()
}
