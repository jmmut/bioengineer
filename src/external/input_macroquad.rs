
use crate::input::{Input, InputSourceTrait, PixelPosition};

use macroquad::input::{
    is_key_down, is_mouse_button_down, is_mouse_button_pressed, is_mouse_button_released,
    mouse_position, mouse_wheel, KeyCode, MouseButton,
};

pub struct InputMacroquad {
    previous_wheel_click_pos: (f32, f32),
}

impl InputMacroquad {
    pub fn new() -> Self {
        InputMacroquad {
            previous_wheel_click_pos: (0.0, 0.0),
        }
    }

    pub fn get_horizontal_move(&mut self) -> PixelPosition {
        let mut diff = PixelPosition::new(0.0, 0.0);
        if is_mouse_button_pressed(MouseButton::Middle) {
            self.previous_wheel_click_pos = mouse_position()
        } else if is_mouse_button_down(MouseButton::Middle) {
            let current_pos = mouse_position();
            diff.x = current_pos.0 - self.previous_wheel_click_pos.0;
            diff.y = current_pos.1 - self.previous_wheel_click_pos.1;

            self.previous_wheel_click_pos = current_pos;
        }
        diff
    }

    pub fn get_left_click_position(&mut self) -> Option<PixelPosition> {
        if is_mouse_button_pressed(MouseButton::Left) {
            let (position_x, position_y) = mouse_position();
            Option::Some(PixelPosition::new(position_x, position_y))
        } else {
            Option::None
        }
    }
    pub fn get_left_click_release(&mut self) -> Option<PixelPosition> {
        if is_mouse_button_released(MouseButton::Left) {
            let (position_x, position_y) = mouse_position();
            Option::Some(PixelPosition::new(position_x, position_y))
        } else {
            Option::None
        }
    }

    fn get_mouse_wheel_height_diff(&mut self) -> i32 {
        let (mouse_x, mouse_y) = mouse_wheel();
        if mouse_x != 0.0 || mouse_y != 0.0 {
            // TODO how can I log in the js console_log?
            // eprintln!("mouse wheel: {}, {}", mouse_x, mouse_y);
        }
        let change_height_rel = if mouse_y > 0.0 {
            1
        } else if mouse_y < 0.0 {
            -1
        } else {
            0
        };
        change_height_rel
    }
}

impl InputSourceTrait for InputMacroquad {
    fn get_input(&mut self) -> Input {
        let quit = is_key_down(KeyCode::Escape);
        let change_height_rel = self.get_mouse_wheel_height_diff();
        let move_map_horizontally = self.get_horizontal_move();
        let start_selection = self.get_left_click_position();
        let end_selection = self.get_left_click_release();
        let (mouse_position_x, mouse_position_y) = mouse_position();
        Input {
            quit,
            change_height_rel,
            move_map_horizontally,
            start_selection,
            end_selection,
            mouse_position: PixelPosition::new(mouse_position_x, mouse_position_y),
        }
    }
}
