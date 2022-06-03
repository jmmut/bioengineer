use crate::drawing;
use crate::input::{Input, InputSourceTrait};
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

    pub fn get_horizontal_move(&mut self) -> (f32, f32) {
        let mut diff = (0.0, 0.0);
        if is_mouse_button_pressed(MouseButton::Middle) {
            self.previous_wheel_click_pos = mouse_position()
        } else if is_mouse_button_down(MouseButton::Middle) {
            let current_pos = mouse_position();
            diff = (
                current_pos.0 - self.previous_wheel_click_pos.0,
                current_pos.1 - self.previous_wheel_click_pos.1,
            );

            self.previous_wheel_click_pos = current_pos;
        }
        diff
    }
}

impl InputSourceTrait for InputMacroquad {
    fn get_input(&mut self) -> Input {
        let quit = is_key_down(KeyCode::Escape);
        let (mouse_x, mouse_y) = mouse_wheel();
        if mouse_x != 0.0 || mouse_y != 0.0 {
            // TODO how can I log in the js console_log?
            eprintln!("mouse wheel: {}, {}", mouse_x, mouse_y);
        }
        let change_height_rel = mouse_y as i32;
        let move_map_horizontally = self.get_horizontal_move();
        Input {
            quit,
            change_height_rel,
            move_map_horizontally,
        }
    }
}
