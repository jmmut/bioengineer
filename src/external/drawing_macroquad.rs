use crate::drawing::{assets, Drawing};
use crate::map::TileType;
use macroquad::color::Color;
use macroquad::prelude::Texture2D;
use macroquad::shapes::draw_rectangle;
use macroquad::text::draw_text;
use macroquad::texture::draw_texture;
use macroquad::window::{clear_background, screen_height, screen_width};

use crate::drawing::DrawingTrait;

pub struct DrawingMacroquad {
    pub drawing: Drawing,
    pub textures: Vec<Texture2D>,
}

impl DrawingTrait for DrawingMacroquad {
    fn new(textures: Vec<Texture2D>) -> DrawingMacroquad {
        // let textures = load_tileset(tileset_path);
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

    // fn draw(&self, game_state: &GameState) {
    // self.debug_draw_all_textures();
    // }

    fn draw_texture(&self, tile: TileType, x: f32, y: f32) {
        self.draw_transparent_texture(tile, x, y, 1.0);
    }
    fn draw_transparent_texture(&self, tile: TileType, x: f32, y: f32, opacity_coef: f32) {
        let mask_color = Color::new(1.0, 1.0, 1.0, opacity_coef);
        draw_texture(self.textures[tile as usize], x, y, mask_color);
    }
    fn draw_colored_texture(&self, tile: TileType, x: f32, y: f32, color_mask: Color) {
        draw_texture(self.textures[tile as usize], x, y, color_mask);
    }
    fn draw_rectangle(&self, x: f32, y: f32, w: f32, h: f32, color: Color) {
        draw_rectangle(x, y, w, h, color);
    }

    fn clear_background(&self, color: Color) {
        clear_background(color);
    }

    fn drawing(&self) -> &Drawing {
        &self.drawing
    }
    fn drawing_mut(&mut self) -> &mut Drawing {
        &mut self.drawing
    }
    fn screen_width(&self) -> f32 {
        screen_width()
    }
    fn screen_height(&self) -> f32 {
        screen_height()
    }
    fn draw_text(&self, text: &str, x: f32, y: f32, font_size: f32, color: Color) {
        draw_text(text, x, y, font_size, color);
    }
}

impl DrawingMacroquad {
    fn _debug_draw_all_textures(&self) {
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
    }
}
