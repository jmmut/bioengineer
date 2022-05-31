use macroquad::prelude::next_frame;

mod drawing;
mod input;

use input::InputSourceTrait;
use input::input_macroquad::InputMacroquad as InputSource;

use drawing::DrawingTrait;
use drawing::drawing_macroquad::DrawingMacroquad as Drawing;


#[macroquad::main("Bioengineer")]
async fn main() {
    let mut frame_index = 0;
    while frame(&frame_index) {
        frame_index = (frame_index + 1) % 1000;
        next_frame().await
    }
}

fn frame(frame_index: &i32) -> bool {
    let input = InputSource::get_input();
    if input.quit {
        false
    } else {
        Drawing::draw(*frame_index);
        true
    }
}
