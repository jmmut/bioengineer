use logic::screen::input_trait::InputTrait;
use macroquad::input::{
    is_key_down, is_key_pressed, is_mouse_button_down, is_mouse_button_pressed,
    is_mouse_button_released, mouse_position, KeyCode, MouseButton,
};
use macroquad::math::Vec2;

pub struct InputMacroquad;

impl InputTrait for InputMacroquad {
    fn is_key_down(&self, key: KeyCode) -> bool {
        is_key_down(key)
    }

    fn is_key_pressed(&self, key: KeyCode) -> bool {
        is_key_pressed(key)
    }

    fn is_mouse_button_down(&self, button: MouseButton) -> bool {
        is_mouse_button_down(button)
    }

    fn is_mouse_button_pressed(&self, button: MouseButton) -> bool {
        is_mouse_button_pressed(button)
    }

    fn is_mouse_button_released(&self, button: MouseButton) -> bool {
        is_mouse_button_released(button)
    }

    fn mouse_position(&self) -> Vec2 {
        Vec2::from(mouse_position())
    }
}
