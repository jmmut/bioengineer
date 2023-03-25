use macroquad::color::Color;
use macroquad::hash;
use macroquad::input::{is_mouse_button_pressed, mouse_position, MouseButton};
use macroquad::math::{Rect, RectOffset, Vec2};
use macroquad::prelude::Texture2D;
use macroquad::shapes::draw_rectangle;
use macroquad::text::{draw_text, measure_text};
use macroquad::texture::{
    draw_texture as macroquad_draw_texture, draw_texture_ex as macroquad_draw_texture_ex,
    DrawTextureParams,
};
use macroquad::ui::widgets::Texture;
use macroquad::ui::{root_ui, widgets, Skin};
use macroquad::window::{clear_background, screen_height, screen_width};

use crate::screen::assets::{PIXELS_PER_TILE_HEIGHT, PIXELS_PER_TILE_WIDTH};
use crate::screen::drawer_trait::{DrawerTrait, Interaction};
use crate::screen::drawing_state::DrawingState;
use crate::screen::gui::{GuiActions, FONT_SIZE, MARGIN};
use crate::screen::{assets, Screen};
use crate::world::map::cell::{TextureIndex, TextureIndexTrait};

pub struct DrawerEguiMiniquad {
    pub drawing: DrawingState,
    pub textures: Vec<Texture2D>,
}

impl DrawerTrait for DrawerEguiMiniquad {
    fn new(textures: Vec<Texture2D>) -> Self {
        // let textures = load_tileset(tileset_path);
        println!(
            "Loaded {} textures. The first one is {} by {} pixels",
            textures.len(),
            textures[0].width(),
            textures[0].height()
        );
        let d = Self {
            drawing: DrawingState::new(),
            textures,
        };
        d._debug_draw_all_textures();
        d
    }

    // fn draw(&self, game_state: &GameState) {
    // self.debug_draw_all_textures();
    // }

    fn screen_width(&self) -> f32 {
        screen_width()
    }
    fn screen_height(&self) -> f32 {
        screen_height()
    }
    fn clear_background(&self, color: Color) {
        clear_background(color);
    }
    fn draw_texture(&self, texture_index: &dyn TextureIndexTrait, x: f32, y: f32) {
        self.draw_transparent_texture(texture_index, x, y, 1.0, 1.0);
    }

    fn draw_transparent_texture(
        &self,
        texture: &dyn TextureIndexTrait,
        x: f32,
        y: f32,
        zoom: f32,
        opacity_coef: f32,
    ) {
        let color_mask = Color::new(1.0, 1.0, 1.0, opacity_coef);
        let texture = self.textures[texture.get_index()];
        macroquad_draw_texture_ex(texture, x, y, color_mask, params_from_zoom(zoom, texture));
    }
    fn draw_colored_texture(
        &self,
        texture: &dyn TextureIndexTrait,
        x: f32,
        y: f32,
        zoom: f32,
        color_mask: Color,
    ) {
        let texture = self.textures[texture.get_index()];
        macroquad_draw_texture_ex(texture, x, y, color_mask, params_from_zoom(zoom, texture));
    }

    fn draw_rectangle(&self, x: f32, y: f32, w: f32, h: f32, color: Color) {
        draw_rectangle(x, y, w, h, color);
    }
    fn draw_text(&self, text: &str, x: f32, y: f32, font_size: f32, color: Color) {
        draw_text(text, x, y, font_size, color);
    }

    fn ui_run(&mut self, f: &mut dyn FnMut() -> ()) {
        f()
    }

    fn ui_draw(&mut self) {
        //TODO: call egui.draw()
    }

    /// This grouping function does not support nested groups
    fn ui_group(&self, x: f32, y: f32, w: f32, h: f32, f: &mut dyn FnMut() -> ()) -> Interaction {
        let id = hash!(x.abs() as i32, y.abs() as i32);
        let window = widgets::Window::new(id, Vec2::new(x, y), Vec2::new(w, h))
            .titlebar(false)
            .movable(false);
        let token = window.begin(&mut root_ui());
        f();
        token.end(&mut root_ui());
        get_interaction(x, y, w, h)
    }

    fn ui_named_group(
        &self,
        title: &str,
        x: f32,
        y: f32,
        w: f32,
        h: f32,
        f: &mut dyn FnMut(),
    ) -> Interaction {
        let id = hash!(title, x.abs() as i32, y.abs() as i32);
        let window = widgets::Window::new(id, Vec2::new(x, y), Vec2::new(w, h))
            .titlebar(true)
            .label(title)
            .movable(false);
        let token = window.begin(&mut root_ui());
        f();
        token.end(&mut root_ui());
        get_interaction(x, y, w, h)
    }

    fn ui_texture(&self, texture_index: TextureIndex) -> bool {
        let clicked = root_ui().texture(
            self.get_texture_copy(texture_index),
            PIXELS_PER_TILE_WIDTH as f32,
            PIXELS_PER_TILE_HEIGHT as f32,
        );
        clicked
        // I would like to do "interaction_from_clicked(clicked)", but
        // for some reason macroquad doesn't register hovering over textures. We would need to put
        // the texture inside a window or a button. For the window we would need pos and size, and
        // the button I think only supports textures as a skin which is tedious.
    }

    fn ui_texture_with_pos(&self, texture_index: &dyn TextureIndexTrait, x: f32, y: f32) -> bool {
        let clicked = Texture::new(self.get_texture_copy(texture_index))
            .size(PIXELS_PER_TILE_WIDTH as f32, PIXELS_PER_TILE_HEIGHT as f32)
            .position(Some(Vec2::new(x, y)))
            .ui(&mut root_ui());
        clicked
        // I would like to do "interaction_from_clicked(clicked)", but
        // for some reason macroquad doesn't register hovering over textures. We would need to put
        // the texture inside a window or a button. For the window we would need pos and size, and
        // the button I think only supports textures as a skin which is tedious.
    }

    fn ui_button(&self, text: &str) -> Interaction {
        let clicked = root_ui().button(None, text);
        interaction_from_clicked(clicked)
    }

    fn ui_button_with_pos(&self, text: &str, x: f32, y: f32) -> Interaction {
        //
        // egui::CentralPanel::default().show(ctx, |ui| {
        //     ui.heading("My egui Application");
        //     ui.horizontal(|ui| {
        //         let name_label = ui.label("Your name: ");
        //         ui.text_edit_singleline(&mut self.name)
        //             .labelled_by(name_label.id);
        //     });
        //     ui.add(egui::Slider::new(&mut self.age, 0..=120).text("age"));
        //     if ui.button("Click each year").clicked() {
        //         self.age += 1;
        //     }
        //     ui.label(format!("Hello '{}', age {}", self.name, self.age));
        // });
        let clicked = root_ui().button(Option::Some(Vec2::new(x, y)), text);
        interaction_from_clicked(clicked)
    }

    fn ui_text(&self, text: &str) {
        root_ui().label(None, text);
    }

    fn measure_text(&self, text: &str, font_size: f32) -> Vec2 {
        let text_dimensions = measure_text(text, Option::None, font_size as u16, 1.0);
        Vec2::new(text_dimensions.width, text_dimensions.height)
    }

    fn ui_same_line(&self) {
        root_ui().same_line(0.0)
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
        // let label_style = root_ui()
        //     .style_builder()
        //     .text_color(text_color)
        //     .font_size(font_size)
        //     .build();
        let margin = RectOffset::new(MARGIN, MARGIN, MARGIN / 5.0, MARGIN / 5.0);
        let button_style = root_ui()
            .style_builder()
            .background_margin(RectOffset::new(0.0, 0.0, 0.0, 0.0))
            .margin(margin.clone())
            .text_color(button_text_color)
            .color(background_color_button)
            .color_hovered(background_color_button_hovered)
            .color_clicked(background_color_button_clicked)
            .color_selected(background_color_button)
            .color_selected_hovered(background_color_button)
            .color_inactive(background_color_button)
            // .font_size(font_size as u16)
            .build();
        let window_style = root_ui()
            .style_builder()
            // .background_margin(margin.clone())
            .margin(margin.clone())
            .text_color(text_color)
            .color(background_color)
            .color_hovered(background_color)
            .color_clicked(background_color)
            .color_inactive(background_color)
            .color_selected(background_color)
            .color_selected_hovered(background_color)
            .font_size(font_size as u16)
            .build();
        let group_style = root_ui()
            .style_builder()
            // .background_margin(margin.clone())
            .margin(margin.clone())
            .text_color(text_color)
            .color(background_color)
            .color_hovered(background_color)
            .color_clicked(background_color)
            .color_inactive(background_color)
            .color_selected(background_color)
            .color_selected_hovered(background_color)
            .font_size(font_size as u16)
            .build();
        let window_titlebar_style = root_ui()
            .style_builder()
            .text_color(text_color)
            .color(background_color)
            .color_hovered(background_color)
            .color_clicked(background_color)
            .color_inactive(background_color)
            .color_selected(background_color)
            .color_selected_hovered(background_color)
            .font_size(font_size as u16)
            .build();
        let label_style = root_ui()
            .style_builder()
            .font_size(font_size as u16)
            .margin(RectOffset::new(
                0.0,
                0.0,
                -font_size / 4.0,
                -font_size / 4.0,
            ))
            .build();
        let skin = Skin {
            // button_style: button_style.clone(),
            button_style,
            window_style,
            group_style,
            window_titlebar_style,
            // window_style: button_style.clone(),
            margin: MARGIN,
            title_height: FONT_SIZE * 2.0,
            label_style,
            ..root_ui().default_skin()
        };
        root_ui().push_skin(&skin);
    }
}

pub fn interaction_from_clicked(clicked: bool) -> Interaction {
    return if clicked {
        Interaction::Clicked
    } else if root_ui().last_item_hovered() {
        Interaction::Hovered
    } else {
        Interaction::None
    };
}

impl DrawerEguiMiniquad {
    fn get_textures(&self) -> &Vec<Texture2D> {
        &self.textures
    }

    fn get_texture_copy<T: Into<TextureIndex>>(&self, texture_index: T) -> Texture2D {
        *self
            .get_textures()
            .get(texture_index.into().get_index())
            .unwrap()
    }

    pub fn _debug_draw_all_textures(&self) {
        for i in 0..self.textures.len() {
            let tiles_per_line = screen_width() as usize / assets::PIXELS_PER_TILE_WIDTH as usize;
            if tiles_per_line > 0 {
                let lines = i / tiles_per_line;
                let x = ((i % tiles_per_line) * assets::PIXELS_PER_TILE_WIDTH as usize) as f32;
                let y = lines as f32 * assets::PIXELS_PER_TILE_HEIGHT as f32;
                let mask_color = Color::new(1.0, 1.0, 1.0, 1.0);
                macroquad_draw_texture(self.textures[i], x, y, mask_color);
            }
        }
    }
}

fn get_interaction(x: f32, y: f32, w: f32, h: f32) -> Interaction {
    let (mouse_x, mouse_y) = mouse_position();
    let group_rect = Rect { x, y, w, h };
    return if group_rect.contains(Vec2::new(mouse_x, mouse_y)) {
        if is_mouse_button_pressed(MouseButton::Left) {
            Interaction::Clicked
        } else {
            // root_ui().focus_window(id);
            Interaction::Hovered
        }
    } else {
        Interaction::None
    };
}

fn params_from_zoom(zoom: f32, texture: Texture2D) -> DrawTextureParams {
    DrawTextureParams {
        dest_size: Some(Vec2::new(texture.height() * zoom, texture.width() * zoom)),
        source: None,
        rotation: 0.0,
        flip_x: false,
        flip_y: false,
        pivot: None,
    }
}

use crate::frame;
use crate::world::World;
use macroquad::miniquad;

pub struct Stage {
    egui_mq: egui_miniquad::EguiMq,
    show_egui_demo_windows: bool,
    egui_demo_windows: egui_demo_lib::DemoWindows,
    color_test: egui_demo_lib::ColorTest,
    pixels_per_point: f32,
    screen: Screen,
    world: World,
}

impl Stage {
    pub fn new(ctx: &mut miniquad::Context, screen: Screen, world: World) -> Self {
        Self {
            egui_mq: egui_miniquad::EguiMq::new(ctx),
            show_egui_demo_windows: true,
            egui_demo_windows: Default::default(),
            color_test: Default::default(),
            pixels_per_point: ctx.dpi_scale(),
            screen,
            world,
        }
    }
}

impl miniquad::EventHandler for Stage {
    fn update(&mut self, _ctx: &mut miniquad::Context) {}

    fn draw(&mut self, mq_ctx: &mut miniquad::Context) {
        mq_ctx.clear(Some((1., 1., 1., 1.)), None, None);
        mq_ctx.begin_default_pass(miniquad::PassAction::clear_color(0.0, 0.0, 0.0, 1.0));
        mq_ctx.end_render_pass();

        let dpi_scale = mq_ctx.dpi_scale();

        let gui_actions = Option::None;
        // Run the UI code:
        self.egui_mq.run(mq_ctx, |_mq_ctx, egui_ctx| {
            if self.show_egui_demo_windows {
                self.egui_demo_windows.ui(egui_ctx);
            }

            egui::Window::new("egui ❤ miniquad").show(egui_ctx, |ui| {
                egui::widgets::global_dark_light_mode_buttons(ui);
                ui.checkbox(&mut self.show_egui_demo_windows, "Show egui demo windows");

                ui.group(|ui| {
                    ui.label("Physical pixels per each logical 'point':");
                    ui.label(format!("native: {:.2}", dpi_scale));
                    ui.label(format!("egui:   {:.2}", ui.ctx().pixels_per_point()));
                    ui.add(
                        egui::Slider::new(&mut self.pixels_per_point, 0.75..=3.0).logarithmic(true),
                    )
                    .on_hover_text("Physical pixels per logical point");
                    if ui.button("Reset").clicked() {
                        self.pixels_per_point = dpi_scale;
                    }
                });

                #[cfg(not(target_arch = "wasm32"))]
                {
                    if ui.button("Quit").clicked() {
                        std::process::exit(0);
                    }
                }
            });

            // Don't change scale while dragging the slider
            if !egui_ctx.is_using_pointer() {
                egui_ctx.set_pixels_per_point(self.pixels_per_point);
            }

            egui::Window::new("Color Test").show(egui_ctx, |ui| {
                egui::ScrollArea::both()
                    .auto_shrink([false; 2])
                    .show(ui, |ui| {
                        self.color_test.ui(ui);
                    });
            });
            let gui_actions = Some(self.screen.get_gui_actions(&self.world));
        });

        // Draw things behind egui here
        let should_continue = self.world.update(gui_actions.unwrap());
        // TODO: how to stop the program?
        self.screen.draw(&self.world);

        // Draw egui
        self.egui_mq.draw(mq_ctx);

        // Draw things in front of egui here

        mq_ctx.commit_frame();
    }

    fn mouse_motion_event(&mut self, _: &mut miniquad::Context, x: f32, y: f32) {
        self.egui_mq.mouse_motion_event(x, y);
    }

    fn mouse_wheel_event(&mut self, _: &mut miniquad::Context, dx: f32, dy: f32) {
        self.egui_mq.mouse_wheel_event(dx, dy);
    }

    fn mouse_button_down_event(
        &mut self,
        ctx: &mut miniquad::Context,
        mb: miniquad::MouseButton,
        x: f32,
        y: f32,
    ) {
        self.egui_mq.mouse_button_down_event(ctx, mb, x, y);
    }

    fn mouse_button_up_event(
        &mut self,
        ctx: &mut miniquad::Context,
        mb: miniquad::MouseButton,
        x: f32,
        y: f32,
    ) {
        self.egui_mq.mouse_button_up_event(ctx, mb, x, y);
    }

    fn char_event(
        &mut self,
        _ctx: &mut miniquad::Context,
        character: char,
        _keymods: miniquad::KeyMods,
        _repeat: bool,
    ) {
        self.egui_mq.char_event(character);
    }

    fn key_down_event(
        &mut self,
        ctx: &mut miniquad::Context,
        keycode: miniquad::KeyCode,
        keymods: miniquad::KeyMods,
        _repeat: bool,
    ) {
        self.egui_mq.key_down_event(ctx, keycode, keymods);
    }

    fn key_up_event(
        &mut self,
        _ctx: &mut miniquad::Context,
        keycode: miniquad::KeyCode,
        keymods: miniquad::KeyMods,
    ) {
        self.egui_mq.key_up_event(keycode, keymods);
    }
}
