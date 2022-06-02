use macroquad::color::colors::GRAY;
use macroquad::color::{BLACK, Color};
use macroquad::miniquad::date::now;
use macroquad::prelude::Texture2D;
use macroquad::text::draw_text;
use macroquad::texture::draw_texture;
use macroquad::window::{clear_background, screen_width};

use super::assets::load_tileset;
use super::DrawingTrait;
use super::super::game_state::GameState;

pub struct DrawingMacroquad {
    pub textures: Vec<Texture2D>,
}

impl DrawingTrait for DrawingMacroquad {
    fn new(tileset_path: &str) -> DrawingMacroquad {
        let textures = load_tileset(tileset_path);
        println!(
            "got {} textures. The first one is {} by {} pixels",
            textures.len(),
            textures[0].width(),
            textures[0].height()
        );
        DrawingMacroquad { textures }
    }

    fn draw(&self, game_state: &GameState) {
        clear_background(GRAY);

        // draw_line(40.0, 40.0, 100.0, 200.0, 15.0, BLUE);
        // draw_rectangle(screen_width() / 2.0 - 60.0, 100.0, 120.0, 60.0, GREEN);
        // draw_circle(screen_width() - 30.0, screen_height() - 30.0, 15.0, YELLOW);
        //
        let text = format!("{:.0}", 1.0/(now() - game_state.previous_frame_ts));
        let font_size = 30.0;
        draw_text(text.as_str(), screen_width() - font_size *2.0, 20.0, font_size, BLACK);
        draw_texture(self.textures[1], 0.0, 0.0, Color::new(1.0, 1.0, 1.0, 1.0));
    }
}
