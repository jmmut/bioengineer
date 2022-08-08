
use crate::Texture2D;
use crate::Color;
use crate::Vec2;
use crate::world::map::TileType;

/// Trait to be implemented by a graphics library.
/// The purpose of this class is to decouple the project from the graphics library.
/// Hopefully, if I ever need to swap the graphics library (currently macroquad), classes like
/// this one will be the only places to change.
/// I'm not sure this will actually help, but we'll see.
pub trait DrawerTrait {
    fn new(textures: Vec<Texture2D>) -> Self
    where
        Self: Sized;

    fn screen_width(&self) -> f32;
    fn screen_height(&self) -> f32;
    fn clear_background(&self, color: Color);
    fn draw_texture(&self, tile: TileType, x: f32, y: f32);
    fn draw_transparent_texture(&self, tile: TileType, x: f32, y: f32, opacity_coef: f32);
    fn draw_colored_texture(&self, tile: TileType, x: f32, y: f32, color_mask: Color);
    fn draw_rectangle(&self, x: f32, y: f32, w: f32, h: f32, color: Color);
    fn draw_text(&self, text: &str, x: f32, y: f32, font_size: f32, color: Color);

    /// both draws and returns if it was pressed. (Immediate mode UI)
    fn do_button(&self, text: &str, x: f32, y: f32) -> bool;
    fn measure_text(&self, text: &str, font_size: f32) -> Vec2;

    fn set_button_style(
        &mut self,
        font_size: f32,
        text_color: Color,
        background_color: Color,
        background_color_hovered: Color,
        background_color_clicked: Color,
    );
}