use crate::screen::assets::{PIXELS_PER_TILE_HEIGHT, PIXELS_PER_TILE_WIDTH};
use crate::screen::drawer_trait::DrawerTrait;
use crate::screen::drawing_state::DrawingState;
use crate::screen::gui::panels::draw_available_transformations::to_action_str;
use crate::screen::gui::panels::longest;
use crate::screen::gui::{GuiActions, FONT_SIZE, MARGIN};
use crate::screen::main_scene_input::CellSelection;
use crate::world::map::cell::{ExtraTextures, TextureIndex};
use crate::world::map::transform_cells::TransformationResult;
use crate::world::{Task, World};
use std::collections::HashSet;

pub fn draw_robot_queue(
    drawer: &mut dyn DrawerTrait,
    world: &World,
    gui_actions: GuiActions,
    drawing: &mut DrawingState,
) -> GuiActions {
    let mut cell_selection = gui_actions.cell_selection;
    let margin = MARGIN;
    let icon_width = PIXELS_PER_TILE_WIDTH as f32 * 1.0;
    let icon_height = PIXELS_PER_TILE_HEIGHT as f32 * 1.0;
    let button_height = FONT_SIZE * 1.5;
    let group_width = icon_width + 3.5 * margin; // for some reason the button has a left margin bigger than MARGIN
    let group_height = icon_height + button_height + 3.0 * margin;
    let go_to_robot = Option::None;

    let mut cancel_task = Option::None;
    for (task_index, task) in world.task_queue.iter().enumerate() {
        let mut cancel_hovered = false;
        let (task_tile, task_description) = match task {
            Task::Transform(transform) => (
                TextureIndex::from(transform.transformation.new_tile_type),
                {
                    let mut description = vec![format!(
                        "Task: {} ({})",
                        to_action_str(transform.transformation.new_tile_type),
                        transform.to_transform.len(),
                    )];
                    description.append(&mut format_reasons(&transform.blocked_because));
                    description
                },
            ),
            Task::Movement(_) => (
                TextureIndex::from(ExtraTextures::Movement),
                vec!["Task: Move".to_owned()],
            ),
        };
        let group = drawer.ui_group(
            drawer.screen_width() - (group_width + margin) * (1 + task_index) as f32,
            drawer.screen_height() - group_height - margin,
            group_width,
            group_height,
            &mut |drawer| {
                let cancel = drawer.ui_button("Cancel");
                cancel_hovered = cancel.is_hovered();
                if cancel.is_clicked() {
                    cancel_task = Option::Some(task_index);
                }

                drawer.ui_texture(task_tile);
            },
        );
        if group.is_hovered_or_clicked() {
            cell_selection = CellSelection::no_selection();
        }

        if cancel_hovered {
            draw_task_queue_tooltip(
                drawer,
                group_height,
                margin,
                &vec!["Stop doing this task".to_string()],
            );
        } else if group.is_hovered_or_clicked() {
            draw_task_queue_tooltip(
                drawer,
                group_height,
                margin,
                task_description.as_slice().as_ref(),
            );
        }
        if group.is_clicked() {
            match task {
                Task::Transform(t) => {
                    drawing.set_highlighted_cells(t.to_transform.clone());
                }
                Task::Movement(_) => {}
            }
        }
    }

    GuiActions {
        go_to_robot,
        cancel_task,
        cell_selection,
        ..gui_actions
    }
}

fn format_reasons(reasons: &Option<HashSet<TransformationResult>>) -> Vec<String> {
    if let Some(reasons) = reasons {
        let mut message = vec!["Blocked because:".to_string()];
        let mut reasons_lines = Vec::new();
        for transformation_result in reasons {
            let reason_line = match transformation_result {
                TransformationResult::Ok => "  should not be printed",
                TransformationResult::NotEnoughMaterial => "  Not enough resources",
                TransformationResult::NotEnoughStorage => "  Not enough storage capacity",
                TransformationResult::AboveWouldCollapse => "  Cells above would collapse",
                TransformationResult::NoSturdyBase => "  Cells below can not support it",
                TransformationResult::OutOfShipReach => "  The spaceship network can't reach",
                TransformationResult::CanNotDeconstructShip => {
                    "  You're not allowed to remove the spaceship"
                }
            }
            .to_string();
            reasons_lines.push(reason_line);
        }
        reasons_lines.sort();
        message.append(&mut reasons_lines);
        message
    } else {
        Vec::new()
    }
}

fn draw_task_queue_tooltip(
    drawer: &mut dyn DrawerTrait,
    group_height: f32,
    margin: f32,
    tooltip: &[String],
) {
    let tooltip_height = FONT_SIZE * 1.25 * tooltip.len() as f32 + MARGIN * 2.0;
    let empty = "".to_string();
    let longest_line = longest(tooltip.iter(), &empty);
    let max_line_width = drawer.ui_measure_text(longest_line.as_str(), FONT_SIZE).x;
    let tooltip_width = max_line_width + 4.0 * MARGIN;
    drawer.ui_group(
        drawer.screen_width() - margin - tooltip_width,
        drawer.screen_height() - group_height - margin - tooltip_height - margin,
        tooltip_width,
        tooltip_height,
        &mut |drawer| {
            for line in tooltip {
                drawer.ui_text(&line);
            }
        },
    );
}
