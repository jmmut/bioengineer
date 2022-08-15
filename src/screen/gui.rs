use crate::screen::assets::{PIXELS_PER_TILE_HEIGHT, PIXELS_PER_TILE_WIDTH};
use coords::cell_pixel::clicked_cell;
use hud::FULL_OPAQUE;
use crate::screen::drawing_state::DrawingState;
use crate::world::game_state::Task;
pub use gui_actions::GuiActions;
use crate::screen::input::Input;
use crate::world::map::TileType;
use crate::Color;
use crate::GameState;
use crate::screen::drawer_trait::DrawerTrait;
use draw_available_transformations::show_available_transformations;

pub mod gui_actions;
pub mod hud;
pub mod draw_available_transformations;
pub mod draw_map;
pub mod coords;

pub struct Gui;

pub const FONT_SIZE: f32 = 20.0;
pub const BLACK: Color = Color::new(0.0, 0.0, 0.0, 1.0);
pub const TEXT_COLOR: Color = BLACK;
pub const TEXT_COLOR_ALARM: Color = Color::new(0.40, 0.0, 0.0, 1.00);
pub const BACKGROUND_UI_COLOR: Color = Color::new(0.3, 0.3, 0.4, 1.0);
pub const BACKGROUND_UI_COLOR_BUTTON: Color = Color::new(0.32, 0.32, 0.42, 1.0);
pub const BACKGROUND_UI_COLOR_HOVERED: Color = Color::new(0.35, 0.35, 0.45, 1.0);
pub const BACKGROUND_UI_COLOR_CLICKED: Color = Color::new(0.25, 0.25, 0.35, 1.0);

impl Gui {
    pub fn new(drawer: &mut impl DrawerTrait) -> Self {
        Self::set_skin(drawer);
        Gui {}
    }
}

impl Gui {
    pub fn receive_actions(
        self: &Self,
        input: Input,
        drawer: &impl DrawerTrait,
        game_state: &GameState,
        drawing: &DrawingState,
    ) -> GuiActions {
        let unhandled_input = GuiActions {
            input,
            selected_cell_transformation: Option::None,
            robot_movement: Option::None,
            go_to_robot: Option::None,
            cancel_task: Option::None,
            do_now_task: Option::None,
        };
        let unhandled_input = show_available_transformations(drawer, game_state, unhandled_input, drawing);
        let unhandled_input = robot_movement_from_pixel_to_cell(drawer, unhandled_input, drawing);
        let unhandled_input = draw_robot_queue(drawer, game_state, unhandled_input);
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
    game_state: &GameState,
    gui_actions: GuiActions,
) -> GuiActions {
    let mut column = 1.0;
    let icon_width = PIXELS_PER_TILE_WIDTH as f32 * 1.5;
    let pixel_height = drawer.screen_height() - PIXELS_PER_TILE_HEIGHT as f32 * 1.0;
    let queue_length = game_state.task_queue.len();
    let panel_width = (queue_length + 1) as f32 * icon_width;
    drawer.draw_rectangle(
        drawer.screen_width() - panel_width,
        drawer.screen_height() - PIXELS_PER_TILE_HEIGHT as f32,
        panel_width,
        PIXELS_PER_TILE_HEIGHT as f32,
        BACKGROUND_UI_COLOR,
    );
    drawer.draw_transparent_texture(
        TileType::Robot,
        drawer.screen_width() - column * icon_width,
        pixel_height,
        FULL_OPAQUE,
    );
    let button_height = FONT_SIZE * 1.5;
    let mut go_to_robot = Option::None;
    if drawer.do_button(
        "show",
        drawer.screen_width() - column * icon_width,
        pixel_height - button_height,
    ) {
        go_to_robot = Option::Some(game_state.robots.first().unwrap().position);
    }

    let mut cancel_task = Option::None;
    let mut do_now_task = Option::None;
    let mut task_index = 0;
    for task in &game_state.task_queue {
        column += 1.0;
        let tile = match task {
            Task::Transform(transform) => transform.transformation.new_tile_type,
            Task::Movement(_) => TileType::Movement,
        };
        drawer.draw_transparent_texture(
            tile,
            drawer.screen_width() - column * icon_width,
            pixel_height,
            FULL_OPAQUE,
        );
        if drawer.do_button(
            "cancel",
            drawer.screen_width() - column * icon_width,
            pixel_height - button_height,
        ) {
            cancel_task = Option::Some(task_index);
        }
        if drawer.do_button(
            "do now",
            drawer.screen_width() - column * icon_width,
            pixel_height - button_height * 2.0,
        ) {
            do_now_task = Option::Some(task_index);
        }
        task_index += 1;
    }
    GuiActions {
        go_to_robot,
        cancel_task,
        do_now_task,
        ..gui_actions
    }
}
