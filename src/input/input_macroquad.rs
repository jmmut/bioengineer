

use macroquad::input::{is_key_down, KeyCode};
use crate::input::{Input, InputSourceTrait};

pub struct InputMacroquad;

impl InputSourceTrait for InputMacroquad {
    fn get_input() -> Input {
        get_input()
    }
}

fn get_input() -> Input {
    let quit = is_key_down(KeyCode::Escape);
    Input { quit }
}


