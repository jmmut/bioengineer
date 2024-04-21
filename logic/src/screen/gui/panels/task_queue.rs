use crate::screen::assets::{PIXELS_PER_TILE_HEIGHT, PIXELS_PER_TILE_WIDTH};
use crate::screen::drawer_trait::DrawerTrait;
use crate::screen::gui::panels::draw_available_transformations::to_action_str;
use crate::screen::gui::{GuiActions, FONT_SIZE, MARGIN};
use crate::screen::main_scene_input::CellSelection;
use crate::world::map::cell::{ExtraTextures, TextureIndex};
use crate::world::{Task, World};

pub fn draw_robot_queue(
    drawer: &mut dyn DrawerTrait,
    world: &World,
    gui_actions: GuiActions,
) -> GuiActions {
    let mut cell_selection = gui_actions.cell_selection;
    let margin = MARGIN;
    let icon_width = PIXELS_PER_TILE_WIDTH as f32 * 1.5;
    let icon_height = PIXELS_PER_TILE_HEIGHT as f32 * 1.5;
    let button_height = FONT_SIZE * 1.5;
    let group_height = icon_height + 2.0 * button_height;
    let go_to_robot = Option::None;

    let mut cancel_task = Option::None;
    let mut do_now_task = Option::None;
    for (task_index, task) in world.task_queue.iter().enumerate() {
        let mut cancel_hovered = false;
        let mut do_now_hovered = false;
        let (task_tile, task_description) = match task {
            Task::Transform(transform) => (
                TextureIndex::from(transform.transformation.new_tile_type),
                format!(
                    "Task: {} ({})",
                    to_action_str(transform.transformation.new_tile_type),
                    transform.to_transform.len()
                ),
            ),
            Task::Movement(_) => (
                TextureIndex::from(ExtraTextures::Movement),
                "Task: Move".to_owned(),
            ),
        };
        let group = drawer.ui_group(
            drawer.screen_width() - (icon_width + margin) * (1 + task_index) as f32,
            drawer.screen_height() - group_height - margin,
            icon_width,
            group_height,
            &mut |drawer| {
                let cancel = drawer.ui_button("Cancel");
                cancel_hovered = cancel.is_hovered();
                if cancel.is_clicked() {
                    cancel_task = Option::Some(task_index);
                }

                let do_now = drawer.ui_button("Do now");
                do_now_hovered = do_now.is_hovered();
                if do_now.is_clicked() {
                    do_now_task = Option::Some(task_index);
                }
                drawer.ui_texture(task_tile);
            },
        );
        if group.is_hovered_or_clicked() {
            cell_selection = CellSelection::no_selection();
        }

        if cancel_hovered {
            draw_task_queue_tooltip(drawer, group_height, margin, "Stop doing this task");
        }
        if do_now_hovered {
            draw_task_queue_tooltip(
                drawer,
                group_height,
                margin,
                "Pause other tasks and do this task now",
            );
        }
        if !cancel_hovered && !do_now_hovered {
            if group.is_hovered_or_clicked() {
                draw_task_queue_tooltip(drawer, group_height, margin, &task_description);
            }
            // TODO: on group.is_clicked, highlight cells
        }
    }

    GuiActions {
        go_to_robot,
        cancel_task,
        do_now_task,
        cell_selection,
        ..gui_actions
    }
}

fn draw_task_queue_tooltip(
    drawer: &mut dyn DrawerTrait,
    group_height: f32,
    margin: f32,
    tooltip: &str,
) {
    let tooltip_height = FONT_SIZE * 2.5;
    let tooltip_width = drawer.ui_measure_text(tooltip, FONT_SIZE).x + 4.0 * MARGIN;
    drawer.ui_group(
        drawer.screen_width() - margin - tooltip_width,
        drawer.screen_height() - group_height - margin - tooltip_height - margin,
        tooltip_width,
        tooltip_height,
        &mut |drawer| drawer.ui_text(tooltip),
    );
}
