use crate::world::map::cell::TextureIndex;
use crate::world::map::TileType;
use crate::Color;
use crate::Texture2D;
use crate::Vec2;

/// Trait to be implemented by a graphics library.
///
/// The ui_* functions won't directly draw, but wait until the end of the frame, to support
/// immediate mode UI.
///
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

    fn ui_group<F: FnOnce()>(&self, x: f32, y: f32, w: f32, h: f32, f: F);
    fn ui_named_group<F: FnOnce()>(&self, title: &str, x: f32, y: f32, w: f32, h: f32, f: F);
    /// both draws and returns if it was pressed. (Immediate mode UI)
    fn ui_texture(&self, texture_index: impl TextureIndex) -> bool;
    fn ui_texture_with_pos(&self, texture_index: impl TextureIndex, x: f32, y: f32) -> bool;
    /// both draws and returns if it was pressed. (Immediate mode UI)
    fn ui_button(&self, text: &str) -> bool;
    fn ui_button_with_pos(&self, text: &str, x: f32, y: f32) -> bool;
    fn measure_text(&self, text: &str, font_size: f32) -> Vec2;

    fn set_button_style(
        &mut self,
        font_size: f32,
        text_color: Color,
        background_color: Color,
        background_color_hovered: Color,
        background_color_clicked: Color,
    );

    // TODO: remove after refactoring GUI
    fn get_textures(&self) -> &Vec<Texture2D>;
}
