use egui_miniquad::EguiMq;
use macroquad::prelude::*;
use miniquad as mq;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::mem::swap;

use crate::external::drawer_macroquad::DrawerMacroquad;
use crate::screen::assets::{PIXELS_PER_TILE_HEIGHT, PIXELS_PER_TILE_WIDTH};
use crate::screen::drawer_trait::{DrawerTrait, Interaction};
use crate::screen::gui::FONT_SIZE;
use crate::world::map::cell::{TextureIndex, TextureIndexTrait};
pub use egui;
use egui::epaint::Shadow;
use egui::style::{Spacing, WidgetVisuals, Widgets};
use egui::{
    emath, Color32, FontFamily, FontId, Frame, Id, Margin, Pos2, Response, Rounding, Sense, Stroke,
    Style, TextureId, Visuals, Widget,
};
pub use macroquad;
use macroquad::miniquad::GraphicsContext;

pub struct DrawerEguiMacroquad<'a> {
    egui_mq: Option<EguiMq>,
    egui_context: Option<egui::Context>,
    egui_ui: Option<&'a mut egui::Ui>,
    input_processor_id: usize,
    inner: Option<DrawerMacroquad>,
    something_clicked: bool,
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
            something_clicked: false,
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
        let something_clicked = self.something_clicked;
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
                            something_clicked,
                        };

                        f(&mut drawer);

                        ref_mut.inner = drawer.inner.take();
                        ref_mut.something_clicked = drawer.something_clicked;
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
        if is_mouse_button_released(MouseButton::Left)
            || is_mouse_button_released(MouseButton::Right)
            || is_mouse_button_released(MouseButton::Middle)
        {
            self.something_clicked = false;
        }
    }

    fn ui_group(
        &mut self,
        x: f32,
        y: f32,
        w: f32,
        h: f32,
        f: &mut dyn FnMut(&mut dyn DrawerTrait) -> (),
    ) -> Interaction {
        self.ui_group_common(None, x, y, w, h, f)
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
        self.ui_group_common(Some(title.to_string()), x, y, w, h, f)
    }

    fn ui_texture(&mut self, texture_index: TextureIndex) -> bool {
        self.ui_texture_common(texture_index)
    }

    // TODO: make positions work
    fn ui_texture_with_pos(
        &mut self,
        texture_index: &dyn TextureIndexTrait,
        _x: f32,
        _y: f32,
    ) -> bool {
        self.ui_texture_common(texture_index.into())
    }

    fn ui_button(&mut self, text: &str) -> Interaction {
        let response = egui::Button::new(text)
            .wrap(false)
            .ui(self.egui_ui.as_mut().unwrap());
        let inter = self.response_to_interaction(Some(response));
        match inter {
            Interaction::Clicked => {
                return Interaction::Clicked;
            }
            Interaction::Hovered => {
                return Interaction::Hovered;
            }
            Interaction::None => {
                return Interaction::None;
            }
        }
    }

    fn ui_button_with_pos(&mut self, text: &str, _x: f32, _y: f32) -> Interaction {
        self.ui_button(text) //TODO: use position
    }

    fn ui_checkbox(&mut self, checked: &mut bool, text: &str) {
        let checkbox = egui::Checkbox::new(checked, text);
        checkbox.ui(self.egui_ui.as_mut().unwrap());
    }

    fn ui_text(&mut self, text: &str) {
        egui::Label::new(text)
            .wrap(false)
            .ui(self.egui_ui.as_mut().unwrap());
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
                something_clicked: self.something_clicked,
            };
            f(&mut drawer);
            self.inner = drawer.inner.take();
            self.something_clicked = drawer.something_clicked;
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
        let widget_visuals_inactive = WidgetVisuals {
            bg_fill: text_color,
            weak_bg_fill: background_color_button,
            rounding: Rounding::same(0.0),
            bg_stroke: Default::default(),
            fg_stroke: Stroke::new(2.0, background_color_button_clicked),
            expansion: 0.0,
        };
        let widget_visuals_hovered = WidgetVisuals {
            bg_fill: text_color,
            weak_bg_fill: background_color_button_hovered,
            bg_stroke: Stroke::new(1.0, background_color_button),
            fg_stroke: Stroke::new(3.0, background_color_button_hovered),
            ..widget_visuals_inactive
        };

        let widget_visuals_clicked = WidgetVisuals {
            bg_fill: text_color,
            weak_bg_fill: background_color_button_clicked,
            bg_stroke: Stroke::new(2.0, background_color_button_hovered),
            fg_stroke: Stroke::new(1.0, background_color_button_hovered),
            ..widget_visuals_inactive
        };

        let widget_visuals = Widgets {
            inactive: widget_visuals_inactive,
            hovered: widget_visuals_hovered,
            active: widget_visuals_clicked,
            ..Default::default()
        };
        let visuals = Visuals {
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
        use egui::TextStyle::*;

        let font = FontId::new(FONT_SIZE - 4.0, FontFamily::Monospace);
        let text_styles: BTreeMap<egui::TextStyle, FontId> = [
            (Heading, FontId::new(FONT_SIZE - 3.0, FontFamily::Monospace)),
            // (Name("Heading2".into()), font.clone()),
            // (Name("Context".into()), font.clone()),
            (Body, font.clone()),
            (Monospace, font.clone()),
            (Button, font.clone()),
            (Small, font.clone()),
        ]
        .into();

        let spacing = Spacing {
            item_spacing: egui::vec2(10.0, 10.0),
            window_margin: Margin::symmetric(20.0, 15.0),
            // menu_margin: Margin::same(6.0),
            button_padding: egui::vec2(8.0, 3.0),
            // indent: 18.0, // match checkbox/radio-button with `button_padding.x + icon_width + icon_spacing`
            // interact_size: egui::vec2(40.0, 18.0),
            // slider_width: 100.0,
            // combo_width: 100.0,
            // text_edit_width: 280.0,
            icon_width: 15.0,
            icon_width_inner: 10.0,
            icon_spacing: 6.0,
            // tooltip_width: 600.0,
            // combo_height: 200.0,
            // scroll_bar_width: 8.0,
            // scroll_handle_min_length: 12.0,
            // scroll_bar_inner_margin: 4.0,
            // scroll_bar_outer_margin: 0.0,
            // indent_ends_with_horizontal_line: false,
            ..Default::default()
        };

        let style = Style {
            visuals,
            text_styles,
            spacing,
            ..Default::default()
        };
        self.egui_mq.as_mut().unwrap().egui_ctx().set_style(style)
    }

    fn debug_ui(&mut self) {
        let mut style = self.egui_context.as_mut().unwrap().style().as_ref().clone();
        style.ui(self.egui_ui.as_mut().unwrap());
        self.egui_context.as_mut().unwrap().set_style(style);
    }
}

impl<'a> DrawerEguiMacroquad<'a> {
    fn response_to_interaction(&mut self, response: Option<Response>) -> Interaction {
        if let Some(response) = response {
            if response.is_pointer_button_down_on() {
                return if self.something_clicked {
                    Interaction::None
                } else {
                    self.something_clicked = true;
                    Interaction::Clicked
                };
            } else if response.hovered() {
                return Interaction::Hovered;
            }
        }
        Interaction::None
    }

    fn ui_group_common(
        &mut self,
        title: Option<String>,
        x: f32,
        y: f32,
        w: f32,
        h: f32,
        f: &mut dyn FnMut(&mut dyn DrawerTrait) -> (),
    ) -> Interaction {
        let id = if let Some(title) = &title {
            Id::new(title).with(x.abs() as i32).with(y.abs() as i32)
        } else {
            Id::new(x.abs() as i32).with(y.abs() as i32)
        };
        let mut egui_context = None;
        swap(&mut egui_context, &mut self.egui_context);
        let should_have_title_bar = title.is_some();
        let rect = emath::Rect::from_min_size(Pos2::new(x, y), emath::Vec2::new(w, h));
        // self.egui_ui.as_mut().unwrap().
        let response = egui::Window::new(title.unwrap_or("".to_string()))
            .id(id)
            .title_bar(should_have_title_bar)
            // .default_rect(rect)
            // .anchor(Align2::anchor_rect(rect), egui::Vec2::new(0.0, 0.0))
            // .frame(Frame::inner_margin())
            .collapsible(false)
            .resizable(false)
            .fixed_rect(rect)
            .show(egui_context.as_ref().unwrap(), |ui| {
                let extra_margin = ui.style().spacing.window_margin;
                ui.set_width(rect.width() - extra_margin.left - extra_margin.right);
                // ui.set_height(ui.available_height());
                let mut drawer = DrawerEguiMacroquad {
                    egui_mq: self.egui_mq.take(),
                    egui_context: egui_context.clone(),
                    egui_ui: Some(ui),
                    input_processor_id: self.input_processor_id,
                    inner: self.inner.take(),
                    something_clicked: self.something_clicked,
                };
                f(&mut drawer);

                self.inner = drawer.inner.take();
                self.something_clicked = drawer.something_clicked;
            });

        swap(&mut egui_context, &mut self.egui_context);
        self.response_to_interaction(response.map(|inner| inner.response))
    }

    // TODO: make positions work
    fn ui_texture_common(&mut self, texture_index: TextureIndex) -> bool {
        let gl_texture_index = self
            .inner
            .as_ref()
            .unwrap()
            .get_texture_copy(texture_index)
            .raw_miniquad_texture_handle()
            .gl_internal_id();
        let size = egui::Vec2::new(PIXELS_PER_TILE_WIDTH as f32, PIXELS_PER_TILE_HEIGHT as f32);

        let image = egui::Image::new(TextureId::User(gl_texture_index as u64), size);
        let image = image.sense(Sense {
            click: true,
            drag: false,
            focusable: true,
        });
        let ui = self.egui_ui.as_mut().unwrap();
        // let extra_margin = ui.style().spacing.window_margin;
        // let space = y - ui.min_rect().size().y - extra_margin.top;
        // ui.add_space(space);
        let response = image.ui(ui);
        self.response_to_interaction(Some(response)).is_clicked()
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
