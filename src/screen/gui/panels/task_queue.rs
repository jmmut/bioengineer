use crate::screen::assets::{PIXELS_PER_TILE_HEIGHT, PIXELS_PER_TILE_WIDTH};
use crate::screen::drawer_trait::DrawerTrait;
use crate::screen::gui::panels::draw_available_transformations::to_action_str;
use crate::screen::gui::{GuiActions, FONT_SIZE, MARGIN};
use crate::screen::input::CellSelection;
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
    let robot_window_height = icon_height + 1.0 * button_height;
    let mut go_to_robot = Option::None;
    let mut robot_hovered = false;
    let group_robot = drawer.ui_group(
        drawer.screen_width() - icon_width - margin,
        drawer.screen_height() - robot_window_height - margin,
        icon_width,
        robot_window_height,
        &mut |drawer| {
            let show_robot = drawer.ui_button("Show");
            let robot_texture_clicked =
                drawer.ui_texture_with_pos(&ExtraTextures::ZoomedRobot, 0.0, button_height * 2.0);
            robot_hovered = show_robot.is_hovered_or_clicked();
            if show_robot.is_clicked() || robot_texture_clicked {
                go_to_robot = Option::Some(world.robots.first().unwrap().position);
            }
        },
    );
    if group_robot.is_hovered_or_clicked() {
        cell_selection = CellSelection::no_selection();
    }

    let draw_tooltip = |tooltip_enabled: bool, tooltip: &str, drawer| {
        if tooltip_enabled {
            draw_task_queue_tooltip(drawer, group_height, margin, tooltip);
        }
    };
    let tooltip = "Move the camera to the robot";
    draw_tooltip(group_robot.is_hovered_or_clicked(), tooltip, drawer);

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
            drawer.screen_width() - icon_width * (2 + task_index) as f32 - margin,
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
            draw_task_queue_tooltip(drawer, group_height, margin, "Pause other tasks and do this task now");
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
    let tooltip_width = drawer.measure_text(tooltip, FONT_SIZE).x + 4.0 * MARGIN;
    drawer.ui_group(
        drawer.screen_width() - margin - tooltip_width,
        drawer.screen_height() - group_height - margin - tooltip_height - margin,
        tooltip_width,
        tooltip_height,
        &mut |drawer| drawer.ui_text(tooltip),
    );
}
