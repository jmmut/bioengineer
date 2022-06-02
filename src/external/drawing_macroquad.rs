use crate::drawing::{assets, Drawing};
use crate::map::TileType;
use macroquad::color::colors::GRAY;
use macroquad::color::{Color, BLACK};
use macroquad::prelude::Texture2D;
use macroquad::text::draw_text;
use macroquad::texture::draw_texture;
use macroquad::window::{clear_background, screen_height, screen_width};

use super::super::game_state::GameState;
use crate::load_tileset;
use crate::drawing::DrawingTrait;

pub struct DrawingMacroquad {
    pub drawing: Drawing,
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
        DrawingMacroquad {
            drawing: Drawing::new(),
            textures,
        }
    }

    fn draw(&self, game_state: &GameState) {
        clear_background(GRAY);

        // draw_line(40.0, 40.0, 100.0, 200.0, 15.0, BLUE);
        // draw_rectangle(screen_width() / 2.0 - 60.0, 100.0, 120.0, 60.0, GREEN);
        // draw_circle(screen_width() - 30.0, screen_height() - 30.0, 15.0, YELLOW);

        let fps = 1.0 / (game_state.current_frame_ts - game_state.previous_frame_ts);
        // println!(
        //     "now - previous ts: {} - {}, fps: {}, frame: {}",
        //     game_state.current_frame_ts, game_state.previous_frame_ts, fps, game_state.frame_index
        // );

        let text = format!("{:.0}", fps);
        let font_size = 30.0;
        draw_text(
            text.as_str(),
            screen_width() - font_size * 2.0,
            20.0,
            font_size,
            BLACK,
        );
        for i in 0..self.textures.len() {
            let tiles_per_line = screen_width() as usize / assets::PIXELS_PER_TILE_WIDTH as usize;
            if tiles_per_line > 0 {
                let lines = i / tiles_per_line;
                let x = ((i % tiles_per_line) * assets::PIXELS_PER_TILE_WIDTH as usize) as f32;
                let y = lines as f32 * assets::PIXELS_PER_TILE_HEIGHT as f32;
                let mask_color = Color::new(1.0, 1.0, 1.0, 1.0);
                draw_texture(self.textures[i], x, y, mask_color);
            }
        }
        self.draw_map(game_state);
    }

    fn draw_texture(&self, tile: TileType, x: f32, y: f32) {
        let mask_color = Color::new(1.0, 1.0, 1.0, 1.0);
        draw_texture(self.textures[tile as usize], x, y, mask_color);
    }

    fn drawing(&self) -> &Drawing {
        &self.drawing
    }
    fn screen_width(&self) -> f32 {
        screen_width()
    }
    fn screen_height(&self) -> f32 {
        screen_height()
    }
}
