use macroquad::color::Color;
use macroquad::math::{RectOffset, Vec2};
use macroquad::prelude::Texture2D;
use macroquad::shapes::draw_rectangle;
use macroquad::text::{draw_text, measure_text};
use macroquad::texture::draw_texture;
use macroquad::ui::{root_ui, Skin};
use macroquad::window::{clear_background, screen_height, screen_width};

use crate::drawing::{assets, DrawerTrait, Drawing};
use crate::gui::FONT_SIZE;
use crate::map::TileType;

pub struct DrawingMacroquad {
    pub drawing: Drawing,
    pub textures: Vec<Texture2D>,
}

impl DrawerTrait for DrawingMacroquad {
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
    fn screen_width(&self) -> f32 {
        screen_width()
    }
    fn screen_height(&self) -> f32 {
        screen_height()
    }
    fn draw_text(&self, text: &str, x: f32, y: f32, font_size: f32, color: Color) {
        draw_text(text, x, y, font_size, color);
    }

    /// both draws and returns if it was pressed. (Immediate mode UI)
    fn do_button(&self, text: &str, x: f32, y: f32) -> bool {
        root_ui().button(Option::Some(Vec2::new(x, y)), text)
    }

    fn measure_text(&self, text: &str, font_size: f32) -> Vec2 {
        let text_dimensions = measure_text(text, Option::None, font_size as u16, 1.0);
        Vec2::new(text_dimensions.width, text_dimensions.height)
    }

    fn set_button_style(
        &mut self,
        font_size: f32,
        text_color: Color,
        background_color: Color,
        background_color_hovered: Color,
        background_color_clicked: Color,
    ) {
        // let label_style = root_ui()
        //     .style_builder()
        //     .text_color(text_color)
        //     .font_size(font_size)
        //     .build();
        let button_style = root_ui()
            .style_builder()
            // StyleBuilder{} // uncomment to get autocompletion of available methods
            .background_margin(RectOffset::new(0.0, 0.0, 0.0, 0.0))
            .margin(RectOffset::new(
                FONT_SIZE,
                FONT_SIZE,
                FONT_SIZE / 5.0,
                FONT_SIZE / 5.0,
            ))
            .text_color(text_color)
            .color(background_color)
            .color_hovered(background_color_hovered)
            .color_clicked(background_color_clicked)
            .font_size(font_size as u16)
            .build();
        let skin = Skin {
            button_style,
            ..root_ui().default_skin()
        };
        root_ui().push_skin(&skin);
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
