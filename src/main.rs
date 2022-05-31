use macroquad::color::{Color, BLACK, GRAY};
use macroquad::prelude::{draw_texture, next_frame};
use macroquad::window::clear_background;

mod assets;
mod drawing;
mod input;

use input::input_macroquad::InputMacroquad as InputSource;
use input::InputSourceTrait;

use drawing::drawing_macroquad::DrawingMacroquad as Drawing;
use drawing::DrawingTrait;

#[macroquad::main("Bioengineer")]
async fn main() {
    let textures = assets::load_tileset("assets/image/tileset.png");
    println!(
        "got {} textures. The first one is {} by {} pixels",
        textures.len(),
        textures[0].width(),
        textures[0].height()
    );
    let mut frame_index = 0;
    while frame(&frame_index) {
        clear_background(GRAY);
        draw_texture(textures[1], 0.0, 0.0, Color::new(1.0, 1.0, 1.0, 1.0));
        frame_index = (frame_index + 1) % 1000;
        next_frame().await
    }
}

fn frame(frame_index: &i32) -> bool {
    let input = InputSource::get_input();
    if input.quit {
        false
    } else {
        // Drawing::draw(*frame_index);
        true
    }
}
