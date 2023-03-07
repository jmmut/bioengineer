use crate::world::map::cell::TextureIndex;
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
    fn draw_texture<T>(&self, texture_index: T, x: f32, y: f32)
    where
        T: Into<TextureIndex>;
    fn draw_transparent_texture<T>(&self, texture: T, x: f32, y: f32, opacity_coef: f32)
    where
        T: Into<TextureIndex>;
    fn draw_colored_texture<T>(&self, texture: T, x: f32, y: f32, color_mask: Color)
    where
        T: Into<TextureIndex>;
    fn draw_rectangle(&self, x: f32, y: f32, w: f32, h: f32, color: Color);
    fn draw_text(&self, text: &str, x: f32, y: f32, font_size: f32, color: Color);

    fn ui_group<F: FnOnce()>(&self, x: f32, y: f32, w: f32, h: f32, f: F) -> Interaction;
    fn ui_named_group<F: FnOnce()>(
        &self,
        title: &str,
        x: f32,
        y: f32,
        w: f32,
        h: f32,
        f: F,
    ) -> Interaction;
    /// both draws and returns if it was pressed. (Immediate mode UI)
    fn ui_texture(&self, texture_index: TextureIndex) -> bool;
    fn ui_texture_with_pos<T>(&self, texture_index: T, x: f32, y: f32) -> bool
    where
        T: Into<TextureIndex>;
    /// both draws and returns if it was pressed or hovered over. (Immediate mode UI)
    fn ui_button(&self, text: &str) -> Interaction;
    fn ui_button_with_pos(&self, text: &str, x: f32, y: f32) -> Interaction;
    fn ui_text(&self, text: &str);
    fn measure_text(&self, text: &str, font_size: f32) -> Vec2;
    fn ui_same_line(&self);

    fn set_style(
        &mut self,
        font_size: f32,
        text_color: Color,
        button_text_color: Color,
        background_color: Color,
        background_color_button: Color,
        background_color_button_hovered: Color,
        background_color_button_clicked: Color,
    );
}

#[derive(Eq, PartialEq)]
pub enum Interaction {
    Clicked,
    Hovered,
    None,
}

impl Interaction {
    pub fn is_clicked(&self) -> bool {
        *self == Interaction::Clicked
    }

    #[allow(unused)]
    pub fn is_hovered(&self) -> bool {
        *self == Interaction::Hovered
    }

    #[allow(unused)]
    pub fn is_hovered_or_clicked(&self) -> bool {
        *self == Interaction::Hovered || *self == Interaction::Clicked
    }
}
