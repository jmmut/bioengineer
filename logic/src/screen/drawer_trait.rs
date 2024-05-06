use crate::world::map::cell::{TextureIndex, TextureIndexTrait};
use mq_basics::{Color, Texture2D, Vec2};
use std::ops::Range;

/// Trait to be implemented by a graphics library.
///
/// The ui_* functions won't directly draw, but wait until the end of the frame, to support
/// immediate mode UI.
///
/// The purpose of this class is to decouple the project from the graphics library.
/// This has actually allowed integrating egui and be able to swap it at runtime.
/// However, both APIs are quite different and request callbacks in different places, so this
/// interface requires the ugly parts of both APIs. For example, the egui implementation requires
/// all ui code to be run in a callback (see ui_run), so the interface has to require it even though
/// the macroquad implementation doesn't require it.
///
/// I'll probably end up just implementing my own UI based on https://github.com/jmmut/juquad.
pub trait DrawerTrait {
    fn new(textures: Vec<Texture2D>) -> Self
    where
        Self: Sized;

    fn set_textures(&mut self, textures: Vec<Texture2D>);
    fn take_textures(self: Box<Self>) -> Vec<Texture2D>;
    fn screen_width(&self) -> f32;
    fn screen_height(&self) -> f32;
    fn clear_background(&self, color: Color);
    fn texture_size(&self, texture_index: &dyn TextureIndexTrait) -> Vec2;
    fn draw_texture(&self, texture_index: &dyn TextureIndexTrait, x: f32, y: f32);

    /// Takes texture by &dyn because of the hot-reloading machinery. You can't pass a struct
    /// with generic methods through the dynamic library boundary. See lib::hot_reload_draw_frame
    fn draw_transparent_texture(
        &self,
        texture: &dyn TextureIndexTrait,
        x: f32,
        y: f32,
        zoom: f32,
        opacity_coef: f32,
    );
    fn draw_colored_texture(
        &self,
        texture: &dyn TextureIndexTrait,
        x: f32,
        y: f32,
        zoom: f32,
        color_mask: Color,
    );
    fn draw_rotated_texture(
        &self,
        texture: &dyn TextureIndexTrait,
        x: f32,
        y: f32,
        zoom: f32,
        color_mask: Color,
        rotation_radians: f32,
    );
    fn draw_rectangle(&self, x: f32, y: f32, w: f32, h: f32, color: Color);
    fn draw_circle(&self, position: Vec2, radius: f32, color: Color);
    fn draw_text(&self, text: &str, x: f32, y: f32, font_size: f32, color: Color);
    fn measure_text(&mut self, text: &str, font_size: f32) -> Vec2;

    /// all ui_* methods need to run inside ui_run. This is a restriction of using egui_miniquad :(
    fn ui_run(&mut self, f: &mut dyn FnMut(&mut dyn DrawerTrait) -> ());
    fn ui_draw(&mut self);
    fn ui_group(
        &mut self,
        x: f32,
        y: f32,
        w: f32,
        h: f32,
        f: &mut dyn FnMut(&mut dyn DrawerTrait) -> (),
    ) -> Interaction;
    fn ui_named_group(
        &mut self,
        title: &str,
        x: f32,
        y: f32,
        w: f32,
        h: f32,
        f: &mut dyn FnMut(&mut dyn DrawerTrait) -> (),
    ) -> Interaction;
    /// both draws and returns if it was pressed. (Immediate mode UI)
    fn ui_texture(&mut self, texture_index: TextureIndex) -> bool;
    fn ui_texture_with_pos(
        &mut self,
        texture_index: &dyn TextureIndexTrait,
        x: f32,
        y: f32,
    ) -> bool;
    /// both draws and returns if it was pressed or hovered over. (Immediate mode UI)
    fn ui_button(&mut self, text: &str) -> Interaction;
    fn ui_button_with_pos(&mut self, text: &str, x: f32, y: f32) -> Interaction;
    fn ui_checkbox(&mut self, checked: &mut bool, text: &str);

    /// Example usage for experiments:
    /// ```ignore
    /// static mut minimum_lines: f32 = 10.0;
    /// drawer.ui_slider(
    ///     drawer.screen_width() * 0.5,
    ///     drawer.screen_height() * 0.5,
    ///     "minimum lines",
    ///     0.0..20.0,
    ///     unsafe {&mut minimum_lines},
    /// );
    /// ```
    fn ui_slider(&mut self, x: f32, y: f32, label: &str, range: Range<f32>, number: &mut f32);
    fn ui_text(&mut self, text: &str);
    fn ui_measure_text(&mut self, text: &str, font_size: f32) -> Vec2;
    fn ui_same_line(&mut self, f: &mut dyn FnMut(&mut dyn DrawerTrait) -> ());

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
    fn debug_ui(&mut self);
}

#[derive(Eq, PartialEq, Copy, Clone)]
pub enum Interaction {
    Pressing,
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
