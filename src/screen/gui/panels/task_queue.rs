use crate::screen::assets::{PIXELS_PER_TILE_HEIGHT, PIXELS_PER_TILE_WIDTH};
use crate::screen::drawer_trait::DrawerTrait;
use crate::screen::gui::{GuiActions, FONT_SIZE, MARGIN};
use crate::world::map::cell::ExtraTextures;
use crate::world::{Task, World};

pub fn draw_robot_queue(
    drawer: &impl DrawerTrait,
    world: &World,
    gui_actions: GuiActions,
) -> GuiActions {
    let margin = MARGIN;
    let icon_width = PIXELS_PER_TILE_WIDTH as f32 * 1.5;
    let icon_height = PIXELS_PER_TILE_HEIGHT as f32 * 1.5;
    let button_height = FONT_SIZE * 1.5;
    let group_height = icon_height + 2.0 * button_height;
    let robot_window_height = icon_height + 1.0 * button_height;
    let mut go_to_robot = Option::None;
    let mut robot_hovered = false;
    drawer.ui_group(
        drawer.screen_width() - icon_width - margin,
        drawer.screen_height() - robot_window_height - margin,
        icon_width,
        robot_window_height,
        || {
            let show_robot = drawer.ui_button("show");
            let robot_texture =
                drawer.ui_texture_with_pos(ExtraTextures::ZoomedRobot, 0.0, button_height * 2.0);
            robot_hovered = show_robot.is_hovered() || robot_texture.is_hovered();
            if show_robot.is_clicked() || robot_texture.is_clicked() {
                go_to_robot = Option::Some(world.robots.first().unwrap().position);
            }
        },
    );

    let tooltip = "Move the camera to the robot";
    draw_task_queue_tooltip(drawer, group_height, margin, robot_hovered, tooltip);

    let mut cancel_task = Option::None;
    let mut do_now_task = Option::None;
    for (task_index, task) in world.task_queue.iter().enumerate() {
        let mut cancel_hovered = false;
        let mut do_now_hovered = false;
        drawer.ui_group(
            drawer.screen_width() - icon_width * (2 + task_index) as f32 - margin,
            drawer.screen_height() - group_height - margin,
            icon_width,
            group_height,
            || {
                let cancel = drawer.ui_button("cancel");
                cancel_hovered = cancel.is_hovered();
                if cancel.is_clicked() {
                    cancel_task = Option::Some(task_index);
                }

                let do_now = drawer.ui_button("do now");
                do_now_hovered = do_now.is_hovered();
                if do_now.is_clicked() {
                    do_now_task = Option::Some(task_index);
                }
                match task {
                    Task::Transform(transform) => {
                        drawer.ui_texture(transform.transformation.new_tile_type)
                    }
                    Task::Movement(_) => drawer.ui_texture(ExtraTextures::Movement),
                };
            },
        );

        let tooltip = "Stop doing this task";
        draw_task_queue_tooltip(drawer, group_height, margin, cancel_hovered, tooltip);
        let tooltip = "Pause other tasks and do this task now";
        draw_task_queue_tooltip(drawer, group_height, margin, do_now_hovered, tooltip);
    }

    GuiActions {
        go_to_robot,
        cancel_task,
        do_now_task,
        ..gui_actions
    }
}

fn draw_task_queue_tooltip(
    drawer: &impl DrawerTrait,
    group_height: f32,
    margin: f32,
    hovered: bool,
    tooltip: &str,
) {
    let tooltip_height = FONT_SIZE * 2.5;
    let tooltip_width = drawer.measure_text(tooltip, FONT_SIZE).x + 4.0 * MARGIN;
    if hovered {
        drawer.ui_group(
            drawer.screen_width() - margin - tooltip_width,
            drawer.screen_height() - group_height - margin - tooltip_height - margin,
            tooltip_width,
            tooltip_height,
            || drawer.ui_text(tooltip),
        );
    }
}
