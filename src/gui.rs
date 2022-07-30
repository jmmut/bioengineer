use std::cmp::max;
use crate::drawing::coords::cell_pixel::clicked_cell;
use crate::drawing::hud;
use crate::input::Input;
use crate::map::transform_cells::Transformation;
use crate::map::{CellIndex, TileType};
use crate::Color;
use crate::{DrawingTrait, GameState};
use crate::drawing::assets::{PIXELS_PER_TILE_HEIGHT, PIXELS_PER_TILE_WIDTH};
use crate::drawing::hud::FULL_OPAQUE;

pub struct Gui;

pub const FONT_SIZE: f32 = 20.0;
pub const BLACK: Color = Color::new(0.0, 0.0, 0.0, 1.0);
pub const TEXT_COLOR: Color = BLACK;
pub const BACKGROUND_UI_COLOR: Color = Color::new(0.3, 0.3, 0.4, 1.0);
pub const BACKGROUND_UI_COLOR_BUTTON: Color = Color::new(0.32, 0.32, 0.42, 1.0);
pub const BACKGROUND_UI_COLOR_HOVERED: Color = Color::new(0.35, 0.35, 0.45, 1.0);
pub const BACKGROUND_UI_COLOR_CLICKED: Color = Color::new(0.25, 0.25, 0.35, 1.0);

impl Gui {
    pub fn new(drawer: &mut impl DrawingTrait) -> Self {
        Self::set_skin(drawer);
        Gui {}
    }
}

pub struct GuiActions {
    pub input: Input,
    pub selected_cell_transformation: Option<Transformation>,
    pub robot_movement: Option<CellIndex>,
    pub go_to_robot: Option<i32>,
    pub cancel_task: Option<usize>,
    pub do_now_task: Option<usize>,
    pub cancel_movement: Option<usize>,
    pub do_now_movement: Option<usize>,
}

impl Gui {
    pub fn receive_actions(
        self: &mut Self,
        input: Input,
        drawer: &impl DrawingTrait,
        game_state: &GameState,
    ) -> GuiActions {
        let unhandled_input = GuiActions {
            input,
            selected_cell_transformation: Option::None,
            robot_movement: Option::None,
            go_to_robot: Option::None,
            cancel_task: Option::None,
            do_now_task: Option::None,
            cancel_movement: Option::None,
            do_now_movement: Option::None,
        };
        let unhandled_input = hud::show_available_actions(drawer, game_state, unhandled_input);
        let unhandled_input =
            robot_movement_from_pixel_to_cell(drawer, game_state, unhandled_input);
         let unhandled_input = draw_robot_queue(drawer, game_state, unhandled_input);
        unhandled_input
    }
    fn set_skin(drawer: &mut impl DrawingTrait) {
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
    drawer: &impl DrawingTrait,
    game_state: &GameState,
    unhandled_input: GuiActions,
) -> GuiActions {
    let mut robot_movement = Option::None;
    if let Option::Some(movement_to_pixel) = unhandled_input.input.robot_movement {
        let movement_to_cell = clicked_cell(
            movement_to_pixel,
            drawer.screen_width(),
            game_state.get_drawing(),
        );
        robot_movement = Option::Some(movement_to_cell);
    }
    GuiActions {
        robot_movement,
        ..unhandled_input
    }
}

pub fn draw_robot_queue(
    drawer: &impl DrawingTrait,
    game_state: &GameState,
    gui_actions: GuiActions,
) -> GuiActions {
    let mut column = 1.0;
    let icon_width = PIXELS_PER_TILE_WIDTH as f32 * 1.5;
    let pixel_height = drawer.screen_height() - PIXELS_PER_TILE_HEIGHT as f32 * 1.0;
    let max_queue = max(game_state.task_queue.len(), game_state.movement_queue.len());
    let panel_width = (max_queue + 1) as f32 * icon_width;
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
        go_to_robot = Option::Some(game_state.robots.first().unwrap().position.y);
    }

    let mut cancel_task = Option::None;
    let mut do_now_task = Option::None;
    let mut task_index = 0;
    for task in &game_state.task_queue {
        column += 1.0;
        drawer.draw_transparent_texture(
            task.transformation.new_tile_type,
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
    column = 1.0;
    let mut cancel_movement = Option::None;
    let mut do_now_movement = Option::None;
    let mut movement_index = 0;
    for _movement in &game_state.movement_queue {
        column += 1.0;
        drawer.draw_transparent_texture(
            TileType::Movement,
            drawer.screen_width() - column * icon_width,
            pixel_height,
            FULL_OPAQUE,
        );
        if drawer.do_button(
            "cancel",
            drawer.screen_width() - column * icon_width,
            pixel_height - button_height,
        ) {
            cancel_movement = Option::Some(movement_index);
        }
        if drawer.do_button(
            "do now",
            drawer.screen_width() - column * icon_width,
            pixel_height - button_height * 2.0,
        ) {
            do_now_movement = Option::Some(task_index);
        }
        movement_index += 1;
    }
    GuiActions {
        go_to_robot,
        cancel_task,
        do_now_task,
        cancel_movement,
        do_now_movement,
        ..gui_actions
    }
}
