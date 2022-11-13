pub mod coords;
pub mod draw_available_transformations;
pub mod gui_actions;

use coords::cell_pixel::clicked_cell;
use draw_available_transformations::show_available_transformations;
pub use gui_actions::GuiActions;

use crate::screen::assets::{PIXELS_PER_TILE_HEIGHT, PIXELS_PER_TILE_WIDTH};
use crate::screen::drawer_trait::DrawerTrait;
use crate::screen::drawing_state::DrawingState;
use crate::screen::input::{CellSelection, Input};
use crate::world::map::cell::ExtraTextures;
use crate::world::map::TileType;
use crate::world::GameGoalState::{Finished, PostFinished};
use crate::world::{format_age, Task};
use crate::World;
use crate::{Color, Rect, Vec2};

pub const FONT_SIZE: f32 = 16.0;
pub const MARGIN: f32 = 10.0;
pub const BLACK: Color = Color::new(0.0, 0.0, 0.0, 1.0);
pub const TEXT_COLOR: Color = BLACK;
pub const TEXT_COLOR_ALARM: Color = Color::new(0.40, 0.0, 0.0, 1.00);
pub const BACKGROUND_UI_COLOR: Color = Color::new(0.3, 0.3, 0.4, 1.0);
pub const BACKGROUND_UI_COLOR_BUTTON: Color = Color::new(0.32, 0.32, 0.42, 1.0);
pub const BACKGROUND_UI_COLOR_HOVERED: Color = Color::new(0.35, 0.35, 0.45, 1.0);
pub const BACKGROUND_UI_COLOR_CLICKED: Color = Color::new(0.25, 0.25, 0.35, 1.0);

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
        drawing: &DrawingState,
    ) -> GuiActions {
        let unhandled_input = GuiActions {
            input,
            selected_cell_transformation: Option::None,
            robot_movement: Option::None,
            go_to_robot: Option::None,
            cancel_task: Option::None,
            do_now_task: Option::None,
            next_game_goal_state: Option::None,
        };
        let unhandled_input =
            show_available_transformations(drawer, world, unhandled_input, drawing);
        let unhandled_input = robot_movement_from_pixel_to_cell(drawer, unhandled_input, drawing);
        let unhandled_input = draw_robot_queue(drawer, world, unhandled_input);
        let unhandled_input = draw_game_finished(drawer, world, unhandled_input);
        unhandled_input
    }
    fn set_skin(drawer: &mut impl DrawerTrait) {
        drawer.set_button_style(
            FONT_SIZE,
            TEXT_COLOR,
            BACKGROUND_UI_COLOR_BUTTON,
            BACKGROUND_UI_COLOR_HOVERED,
            BACKGROUND_UI_COLOR_CLICKED,
        );
    }
}

fn robot_movement_from_pixel_to_cell(
    drawer: &impl DrawerTrait,
    unhandled_input: GuiActions,
    drawing: &DrawingState,
) -> GuiActions {
    let mut robot_movement = Option::None;
    if let Option::Some(movement_to_pixel) = unhandled_input.input.robot_movement {
        let movement_to_cell = clicked_cell(movement_to_pixel, drawer.screen_width(), drawing);
        robot_movement = Option::Some(movement_to_cell);
    }
    GuiActions {
        robot_movement,
        ..unhandled_input
    }
}

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
        let tile = match task {
            Task::Transform(transform) => transform.transformation.new_tile_type,
            Task::Movement(_) => TileType::Movement,
        };
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
                drawer.ui_texture(tile);
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

pub fn draw_game_finished(
    drawer: &impl DrawerTrait,
    world: &World,
    gui_actions: GuiActions,
) -> GuiActions {
    let mut input = gui_actions.input;
    let next_game_goal_state = if let Finished(age) = world.goal_state {
        input.cell_selection = CellSelection::no_selection();
        input.robot_movement = None;
        let panel_title = "You won!";
        let time_spent = format!("Time spent: {}", format_age(age));
        let text_size_title = drawer.measure_text(panel_title, FONT_SIZE);
        let text_size_age = drawer.measure_text(&time_spent, FONT_SIZE);
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
        drawer.ui_named_group(panel_title, panel.x, panel.y, panel.w, panel.h, || {
            drawer.ui_text(&time_spent);
            if drawer.ui_button("Continue").is_clicked() {
                new_state = Some(PostFinished)
            }
        });
        new_state

        // TODO: add restarted state
    } else {
        None
    };
    GuiActions {
        next_game_goal_state,
        input,
        ..gui_actions
    }
}
