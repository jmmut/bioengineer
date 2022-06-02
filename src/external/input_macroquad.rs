use crate::input::{Input, InputSourceTrait};
use macroquad::input::{is_key_down, is_mouse_button_pressed, KeyCode, mouse_wheel};

pub struct InputMacroquad;

impl InputSourceTrait for InputMacroquad {
    fn get_input() -> Input {
        get_input()
    }
}

fn get_input() -> Input {
    let quit = is_key_down(KeyCode::Escape);
    let (mouse_x, mouse_y) = mouse_wheel();
    let change_height_rel = mouse_y as i32;
    Input { quit, change_height_rel }
}
