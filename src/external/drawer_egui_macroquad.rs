use std::cell::{Cell, RefCell};
use std::mem;
use std::mem::swap;
use std::ops::DerefMut;
use egui_miniquad::EguiMq;
use macroquad::prelude::*;
use miniquad as mq;

use crate::external::drawer_macroquad::DrawerMacroquad;
use crate::screen::drawer_trait::{DrawerTrait, Interaction};
use crate::world::map::cell::{TextureIndex, TextureIndexTrait};
pub use egui;
use egui::{Color32, emath, Frame, Id, Pos2, Response, Ui, Widget};
pub use macroquad;
use macroquad::hash;
use macroquad::miniquad::GraphicsContext;

pub struct DrawerEguiMacroquad<'a> {
    egui_mq: Option<EguiMq>,
    egui_context: Option<egui::Context>,
    egui_ui: Option<&'a mut egui::Ui>,
    input_processor_id: usize,
    inner: Option<DrawerMacroquad>,
}


impl<'a> DrawerTrait for DrawerEguiMacroquad<'a> {
    fn new(textures: Vec<Texture2D>) -> Self {
        Self {
            egui_mq: Some(EguiMq::new(unsafe { get_internal_gl() }.quad_context)),
            egui_context: None,
            egui_ui: None,
            input_processor_id: macroquad::input::utils::register_input_subscriber(),
            // inner: RefCell::new(DrawerMacroquad::new(textures)),
            inner: Some(DrawerMacroquad::new(textures)),
        }
    }

    fn screen_width(&self) -> f32 {
        // self.inner.borrow().screen_width()
        self.inner.as_ref().unwrap().screen_width()
    }

    fn screen_height(&self) -> f32 {
        // self.inner.borrow().screen_height()
        self.inner.as_ref().unwrap().screen_height()
    }

    fn clear_background(&self, color: Color) {
        // self.inner.borrow().clear_background(color)
        self.inner.as_ref().unwrap().clear_background(color)
    }

    fn draw_texture(&self, texture_index: &dyn TextureIndexTrait, x: f32, y: f32) {
        // self.inner.borrow().draw_texture(texture_index, x, y)
        self.inner.as_ref().unwrap().draw_texture(texture_index, x, y)
    }

    fn draw_transparent_texture(
        &self,
        texture: &dyn TextureIndexTrait,
        x: f32,
        y: f32,
        zoom: f32,
        opacity_coef: f32,
    ) {
        // self.inner.borrow()
        self.inner.as_ref().unwrap()
            .draw_transparent_texture(texture, x, y, zoom, opacity_coef)
    }

    fn draw_colored_texture(
        &self,
        texture: &dyn TextureIndexTrait,
        x: f32,
        y: f32,
        zoom: f32,
        color_mask: Color,
    ) {
        // self.inner.borrow()
        self.inner.as_ref().unwrap()
            .draw_colored_texture(texture, x, y, zoom, color_mask)
    }

    fn draw_rectangle(&self, x: f32, y: f32, w: f32, h: f32, color: Color) {
        // self.inner.borrow().draw_rectangle(x, y, w, h, color)
        self.inner.as_ref().unwrap().draw_rectangle(x, y, w, h, color)
    }

    fn draw_text(&self, text: &str, x: f32, y: f32, font_size: f32, color: Color) {
        // self.inner.borrow().draw_text(text, x, y, font_size, color)
        self.inner.as_ref().unwrap().draw_text(text, x, y, font_size, color)
    }

    fn ui_run(&mut self, f: &mut dyn FnMut(&mut dyn DrawerTrait) -> ()) {
        let gl = unsafe { get_internal_gl() };
        macroquad::input::utils::repeat_all_miniquad_input(self, self.input_processor_id);
        let mut egui_mq = None;
        swap(&mut egui_mq, &mut self.egui_mq);
        let ref_cell_self = RefCell::new(self);
        egui_mq.as_mut().unwrap().run(
            gl.quad_context,
            |_: &mut GraphicsContext, egui_context: &egui::Context| {
                egui::CentralPanel::default().frame(Frame {
                    fill: Color32::TRANSPARENT,
                    ..Default::default()
                })
                    .show(egui_context, |ui| {
                        let mut ref_mut = ref_cell_self.borrow_mut();
                        let mut drawer = DrawerEguiMacroquad {
                            egui_mq: None,
                            egui_context: Some(egui_context.clone()),
                            egui_ui: Some(ui),
                            input_processor_id: 0,
                            inner: ref_mut.inner.take(),
                        };

                        f(&mut drawer);

                        ref_mut.inner = drawer.inner.take();
                    });
            },
        );
        swap(&mut egui_mq, &mut ref_cell_self.borrow_mut().egui_mq);
    }


    fn ui_draw(&mut self) {
        let mut gl = unsafe { get_internal_gl() };
        // Ensure that macroquad's shapes are not going to be lost, and draw them now
        gl.flush();
        self.egui_mq.as_mut().unwrap().draw(&mut gl.quad_context);
    }

    fn ui_group(&mut self, x: f32, y: f32, w: f32, h: f32, f: &mut dyn FnMut(&mut dyn DrawerTrait) -> ()) -> Interaction {
        let id = Id::new(x.abs() as i32).with(y.abs() as i32);
        let mut egui_context = None;
        swap(&mut egui_context, &mut self.egui_context);
        let response = egui::Window::new("")
            .id(id)
            .title_bar(false)
            .default_rect(emath::Rect::from_min_size(Pos2::new(x, y), emath::Vec2::new(w, h)))
            .show(egui_context.as_ref().unwrap(), |ui| {
                let mut drawer = DrawerEguiMacroquad {
                    egui_mq: self.egui_mq.take(),
                    egui_context: egui_context.clone(),
                    egui_ui: Some(ui),
                    input_processor_id: self.input_processor_id,
                    inner: self.inner.take(),
                };
                f(&mut drawer);

                self.inner = drawer.inner.take();
            });

        swap(&mut egui_context, &mut self.egui_context);
        Self::response_to_interaction(response.map(|inner| {inner.response}))
    }

    fn ui_named_group(
        &mut self,
        title: &str,
        x: f32,
        y: f32,
        w: f32,
        h: f32,
        f: &mut dyn FnMut(&mut dyn DrawerTrait) -> (),
    ) -> Interaction {
        let id = Id::new(title).with(x.abs() as i32).with(y.abs() as i32);
        let mut egui_context = None;
        swap(&mut egui_context, &mut self.egui_context);
        let response = egui::Window::new(title)
            .id(id)
            .title_bar(false)
            .default_rect(emath::Rect::from_min_size(Pos2::new(x, y), emath::Vec2::new(w, h)))
            .show(egui_context.as_ref().unwrap(), |ui| {
                let mut drawer = DrawerEguiMacroquad {
                    egui_mq: self.egui_mq.take(),
                    egui_context: egui_context.clone(),
                    egui_ui: Some(ui),
                    input_processor_id: self.input_processor_id,
                    inner: self.inner.take(),
                };
                f(&mut drawer);

                self.inner = drawer.inner.take();
            });

        swap(&mut egui_context, &mut self.egui_context);
        Self::response_to_interaction(response.map(|inner| {inner.response}))
    }

    fn ui_texture(&self, texture_index: TextureIndex) -> bool {
        // TODO
        false
    }

    fn ui_texture_with_pos(&self, texture_index: &dyn TextureIndexTrait, x: f32, y: f32) -> bool {
        // TODO
        false
    }

    fn ui_button(&mut self, text: &str) -> Interaction {
        let response = egui::Button::new(text).ui(self.egui_ui.as_mut().unwrap());
        Self::response_to_interaction(Some(response))
    }

    fn ui_button_with_pos(&mut self, text: &str, x: f32, y: f32) -> Interaction {
        self.ui_button(text)
    }

    fn ui_text(&self, text: &str) {
        egui::CentralPanel::default().show(self.egui_context.as_ref().unwrap(), |ui|{
            ui.label(text)
        });
    }

    fn measure_text(&self, text: &str, font_size: f32) -> Vec2 {
        self.inner.as_ref().unwrap().measure_text(text, font_size)
    }

    fn ui_same_line(&self) {
        //TODO
    }

    fn set_style(
        &mut self,
        font_size: f32,
        text_color: Color,
        button_text_color: Color,
        background_color: Color,
        background_color_button: Color,
        background_color_button_hovered: Color,
        background_color_button_clicked: Color,
    ) {
        self.inner.as_mut().unwrap().set_style(
            font_size,
            text_color,
            button_text_color,
            background_color,
            background_color_button,
            background_color_button_hovered,
            background_color_button_clicked,
        )
    }
}

impl<'a> DrawerEguiMacroquad<'a> {
    fn response_to_interaction(response: Option<Response>) -> Interaction {
        if let Some(response) = response {
            if response.clicked() {
                return Interaction::Clicked
            } else if response.hovered() {
                return Interaction::Hovered
            }
        }
        Interaction::None
    }
}

impl<'a> mq::EventHandler for DrawerEguiMacroquad<'a> {
    fn update(&mut self, _ctx: &mut mq::Context) {}

    fn draw(&mut self, _ctx: &mut mq::Context) {}

    fn mouse_motion_event(&mut self, _ctx: &mut mq::Context, x: f32, y: f32) {
        self.egui_mq.as_mut().unwrap().mouse_motion_event(x, y);
    }

    fn mouse_wheel_event(&mut self, _ctx: &mut mq::Context, dx: f32, dy: f32) {
        self.egui_mq.as_mut().unwrap().mouse_wheel_event(dx, dy);
    }

    fn mouse_button_down_event(
        &mut self,
        ctx: &mut mq::Context,
        mb: mq::MouseButton,
        x: f32,
        y: f32,
    ) {
        self.egui_mq.as_mut().unwrap().mouse_button_down_event(ctx, mb, x, y);
    }

    fn mouse_button_up_event(
        &mut self,
        ctx: &mut mq::Context,
        mb: mq::MouseButton,
        x: f32,
        y: f32,
    ) {
        self.egui_mq.as_mut().unwrap().mouse_button_up_event(ctx, mb, x, y);
    }

    fn char_event(
        &mut self,
        _ctx: &mut mq::Context,
        character: char,
        _keymods: mq::KeyMods,
        _repeat: bool,
    ) {
        self.egui_mq.as_mut().unwrap().char_event(character);
    }

    fn key_down_event(
        &mut self,
        ctx: &mut mq::Context,
        keycode: mq::KeyCode,
        keymods: mq::KeyMods,
        _repeat: bool,
    ) {
        self.egui_mq.as_mut().unwrap().key_down_event(ctx, keycode, keymods);
    }

    fn key_up_event(&mut self, _ctx: &mut mq::Context, keycode: mq::KeyCode, keymods: mq::KeyMods) {
        self.egui_mq.as_mut().unwrap().key_up_event(keycode, keymods);
    }
}
