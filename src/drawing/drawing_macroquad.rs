use macroquad::color::colors::{BLACK, BLUE, GRAY, GREEN, YELLOW};
use macroquad::shapes::{draw_circle, draw_line, draw_rectangle};
use macroquad::text::draw_text;
use macroquad::window::{clear_background, screen_height, screen_width};
use crate::DrawingTrait;

pub(crate) struct DrawingMacroquad;

impl DrawingTrait for DrawingMacroquad {
    fn draw(frame_index: i32) {
        draw(frame_index)
    }
}

pub fn draw(frame_index: i32) {
    clear_background(GRAY);

    draw_line(40.0, 40.0, 100.0, 200.0, 15.0, BLUE);
    draw_rectangle(screen_width() / 2.0 - 60.0, 100.0, 120.0, 60.0, GREEN);
    draw_circle(screen_width() - 30.0, screen_height() - 30.0, 15.0, YELLOW);

    let text = format!("IT WORKS! at frame {}", frame_index);
    draw_text(text.as_str(), 20.0, 20.0, 30.0, BLACK);
}
