use egui_miniquad::EguiMq;
use macroquad::prelude::*;
use miniquad as mq;
use std::cell::{RefCell};
use std::mem::swap;

use crate::external::drawer_macroquad::DrawerMacroquad;
use crate::screen::assets::{PIXELS_PER_TILE_HEIGHT, PIXELS_PER_TILE_WIDTH};
use crate::screen::drawer_trait::{DrawerTrait, Interaction};
use crate::world::map::cell::{TextureIndex, TextureIndexTrait};
pub use egui;
use egui::epaint::Shadow;
use egui::style::{WidgetVisuals, Widgets};
use egui::{
    emath, Color32, Frame, Id, Pos2, Response, Rounding, Stroke, Style, TextureId, Visuals,
    Widget,
};
pub use macroquad;
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
        self.inner
            .as_ref()
            .unwrap()
            .draw_texture(texture_index, x, y)
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
        self.inner
            .as_ref()
            .unwrap()
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
        self.inner
            .as_ref()
            .unwrap()
            .draw_colored_texture(texture, x, y, zoom, color_mask)
    }

    fn draw_rectangle(&self, x: f32, y: f32, w: f32, h: f32, color: Color) {
        // self.inner.borrow().draw_rectangle(x, y, w, h, color)
        self.inner
            .as_ref()
            .unwrap()
            .draw_rectangle(x, y, w, h, color)
    }

    fn draw_text(&self, text: &str, x: f32, y: f32, font_size: f32, color: Color) {
        // self.inner.borrow().draw_text(text, x, y, font_size, color)
        self.inner
            .as_ref()
            .unwrap()
            .draw_text(text, x, y, font_size, color)
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
                egui::CentralPanel::default()
                    .frame(Frame {
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

    fn ui_group(
        &mut self,
        x: f32,
        y: f32,
        w: f32,
        h: f32,
        f: &mut dyn FnMut(&mut dyn DrawerTrait) -> (),
    ) -> Interaction {
        let id = Id::new(x.abs() as i32).with(y.abs() as i32);
        let mut egui_context = None;
        swap(&mut egui_context, &mut self.egui_context);
        let response = egui::Window::new("")
            .id(id)
            .title_bar(false)
            .default_rect(emath::Rect::from_min_size(
                Pos2::new(x, y),
                emath::Vec2::new(w, h),
            ))
            .resizable(false)
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
        Self::response_to_interaction(response.map(|inner| inner.response))
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
            .title_bar(true)
            .default_rect(emath::Rect::from_min_size(
                Pos2::new(x, y),
                emath::Vec2::new(w, h),
            ))
            .resizable(false)
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
        Self::response_to_interaction(response.map(|inner| inner.response))
    }

    fn ui_texture(&mut self, texture_index: TextureIndex) -> bool {
        let gl_texture_index = self
            .inner
            .as_ref()
            .unwrap()
            .get_texture_copy(texture_index)
            .raw_miniquad_texture_handle()
            .gl_internal_id();
        let size = egui::Vec2::new(
            PIXELS_PER_TILE_WIDTH as f32 * 2.0,
            PIXELS_PER_TILE_HEIGHT as f32 * 2.0,
        );

        let image = egui::ImageButton::new(TextureId::User(gl_texture_index as u64), size);
        let ui = self.egui_ui.as_mut().unwrap();
        let response = image.ui(ui);
        Self::response_to_interaction(Some(response)).is_clicked()
    }

    fn ui_texture_with_pos(
        &mut self,
        texture_index: &dyn TextureIndexTrait,
        x: f32,
        y: f32,
    ) -> bool {
        let gl_texture_index = self
            .inner
            .as_ref()
            .unwrap()
            .get_texture_copy(texture_index)
            .raw_miniquad_texture_handle()
            .gl_internal_id();
        let size = egui::Vec2::new(
            PIXELS_PER_TILE_WIDTH as f32 * 2.0,
            PIXELS_PER_TILE_HEIGHT as f32 * 2.0,
        );

        let image = egui::ImageButton::new(TextureId::User(gl_texture_index as u64), size);
        let ui = self.egui_ui.as_mut().unwrap();
        let response = image.ui(ui);
        Self::response_to_interaction(Some(response)).is_clicked()
    }

    fn ui_button(&mut self, text: &str) -> Interaction {
        let response = egui::Button::new(text).ui(self.egui_ui.as_mut().unwrap());
        Self::response_to_interaction(Some(response))
    }

    fn ui_button_with_pos(&mut self, text: &str, x: f32, y: f32) -> Interaction {
        self.ui_button(text) //TODO: use position
    }

    fn ui_text(&mut self, text: &str) {
        self.egui_ui.as_mut().unwrap().label(text);
    }

    fn measure_text(&self, text: &str, font_size: f32) -> Vec2 {
        self.inner.as_ref().unwrap().measure_text(text, font_size)
    }

    fn ui_same_line(&mut self, f: &mut dyn FnMut(&mut dyn DrawerTrait) -> ()) {
        let egui_context = self.egui_context.clone();
        self.egui_ui.as_mut().unwrap().horizontal_top(|ui| {
            let mut drawer = DrawerEguiMacroquad {
                egui_mq: self.egui_mq.take(),
                egui_context,
                egui_ui: Some(ui),
                input_processor_id: self.input_processor_id,
                inner: self.inner.take(),
            };
            f(&mut drawer);
            self.inner = drawer.inner.take();
        });
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
        );
        let to_egui_color = |c: Color| -> Color32 {
            let array: [u8; 4] = c.into();
            Color32::from_rgba_premultiplied(array[0], array[1], array[2], array[3])
        };

        let text_color = to_egui_color(text_color);
        // let button_text_color = to_egui_color(button_text_color);
        let background_color = to_egui_color(background_color);
        let background_color_button = to_egui_color(background_color_button);
        let background_color_button_hovered = to_egui_color(background_color_button_hovered);
        let background_color_button_clicked = to_egui_color(background_color_button_clicked);
        let mut style = Style::default();
        let widget_visuals_inactive = WidgetVisuals {
            bg_fill: background_color_button,
            weak_bg_fill: background_color_button,
            bg_stroke: Default::default(),
            rounding: Rounding::same(0.0),
            fg_stroke: Stroke::new(1.0, background_color_button),
            expansion: 0.0,
        };
        let widget_visuals_hovered = WidgetVisuals {
            bg_fill: background_color_button_hovered,
            weak_bg_fill: background_color_button_hovered,
            bg_stroke: Stroke::new(1.0, background_color_button),
            fg_stroke: Stroke::new(1.0, background_color_button_hovered),
            ..widget_visuals_inactive
        };

        let widget_visuals_clicked = WidgetVisuals {
            bg_fill: background_color_button_clicked,
            weak_bg_fill: background_color_button_clicked,
            bg_stroke: Stroke::new(2.0, background_color_button_hovered),
            fg_stroke: Stroke::new(1.0, background_color_button_clicked),
            ..widget_visuals_inactive
        };

        let widget_visuals = Widgets {
            inactive: widget_visuals_inactive,
            hovered: widget_visuals_hovered,
            active: widget_visuals_clicked,
            ..Default::default()
        };
        style.visuals = Visuals {
            dark_mode: false,
            override_text_color: Some(text_color),
            // faint_bg_color: background_color,
            extreme_bg_color: background_color,
            code_bg_color: background_color,
            window_rounding: Rounding::same(0.0),
            window_shadow: Shadow::NONE,
            window_fill: background_color,
            widgets: widget_visuals,
            // button_frame: true,
            ..Default::default()
        };
        self.egui_mq.as_mut().unwrap().egui_ctx().set_style(style)
    }
}

impl<'a> DrawerEguiMacroquad<'a> {
    fn response_to_interaction(response: Option<Response>) -> Interaction {
        if let Some(response) = response {
            if response.clicked() {
                return Interaction::Clicked;
            } else if response.hovered() {
                return Interaction::Hovered;
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
        self.egui_mq
            .as_mut()
            .unwrap()
            .mouse_button_down_event(ctx, mb, x, y);
    }

    fn mouse_button_up_event(
        &mut self,
        ctx: &mut mq::Context,
        mb: mq::MouseButton,
        x: f32,
        y: f32,
    ) {
        self.egui_mq
            .as_mut()
            .unwrap()
            .mouse_button_up_event(ctx, mb, x, y);
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
        self.egui_mq
            .as_mut()
            .unwrap()
            .key_down_event(ctx, keycode, keymods);
    }

    fn key_up_event(&mut self, _ctx: &mut mq::Context, keycode: mq::KeyCode, keymods: mq::KeyMods) {
        self.egui_mq
            .as_mut()
            .unwrap()
            .key_up_event(keycode, keymods);
    }
}
