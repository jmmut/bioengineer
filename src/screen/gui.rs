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
use crate::world::Task;
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
    drawer.ui_group(
        drawer.screen_width() - icon_width - margin,
        drawer.screen_height() - robot_window_height - margin,
        icon_width,
        robot_window_height,
        || {
            let show_robot_clicked = drawer.ui_button("show");
            let robot_texture_clicked =
                drawer.ui_texture_with_pos(ExtraTextures::ZoomedRobot, 0.0, button_height * 2.0);
            if show_robot_clicked || robot_texture_clicked {
                go_to_robot = Option::Some(world.robots.first().unwrap().position);
            }
        },
    );

    let mut cancel_task = Option::None;
    let mut do_now_task = Option::None;
    for (task_index, task) in world.task_queue.iter().enumerate() {
        let tile = match task {
            Task::Transform(transform) => transform.transformation.new_tile_type,
            Task::Movement(_) => TileType::Movement,
        };
        drawer.ui_group(
            drawer.screen_width() - icon_width * (2 + task_index) as f32 - margin,
            drawer.screen_height() - group_height - margin,
            icon_width,
            group_height,
            || {
                if drawer.ui_button("cancel") {
                    cancel_task = Option::Some(task_index);
                }

                if drawer.ui_button("do now") {
                    do_now_task = Option::Some(task_index);
                }
                drawer.ui_texture(tile);
            },
        );
    }

    GuiActions {
        go_to_robot,
        cancel_task,
        do_now_task,
        ..gui_actions
    }
}

pub fn draw_game_finished(
    drawer: &impl DrawerTrait,
    world: &World,
    gui_actions: GuiActions,
) -> GuiActions {
    let mut input = gui_actions.input;
    let next_game_goal_state = if world.goal_state == Finished {
        input.cell_selection = CellSelection::no_selection();
        input.robot_movement = None;
        let panel_title = "You won!";
        let text_size = drawer.measure_text(panel_title, FONT_SIZE);
        let width_by_title = text_size.x * 3.0;
        let height_per_line = text_size.y * 2.0;
        let center = Vec2::new(drawer.screen_width() / 2.0, drawer.screen_height() / 2.0);

        let panel = Rect::new(
            center.x - width_by_title / 2.0,
            center.y - height_per_line * 2.0,
            width_by_title,
            height_per_line * 5.0,
        );
        drawer.draw_rectangle(panel.x, panel.y, panel.w, panel.h, BACKGROUND_UI_COLOR);

        drawer.draw_text(
            panel_title,
            center.x - text_size.x / 2.0,
            center.y - height_per_line,
            FONT_SIZE,
            TEXT_COLOR,
        );
        if drawer.ui_button_with_pos(
            "Continue",
            center.x - text_size.x / 2.0,
            center.y + height_per_line,
        ) {
            Some(PostFinished)
        } else {
            None
        }
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
