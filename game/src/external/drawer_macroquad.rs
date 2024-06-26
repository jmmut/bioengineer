use macroquad::color::Color;
use macroquad::hash;
use macroquad::input::{is_mouse_button_pressed, mouse_position, MouseButton};
use macroquad::math::{vec2, Rect, RectOffset, Vec2};
use macroquad::prelude::Texture2D;
use macroquad::shapes::{draw_circle, draw_rectangle};
use macroquad::text::{draw_text, measure_text};
use macroquad::texture::{
    draw_texture as macroquad_draw_texture, draw_texture_ex as macroquad_draw_texture_ex,
    DrawTextureParams,
};
use macroquad::ui::widgets::Texture;
use macroquad::ui::{root_ui, widgets, Skin};
use macroquad::window::{clear_background, screen_height, screen_width};
use std::ops::Range;

use logic::screen::assets;
use logic::screen::assets::{PIXELS_PER_TILE_HEIGHT, PIXELS_PER_TILE_WIDTH};
use logic::screen::drawer_trait::{DrawerTrait, Interaction};
use logic::screen::drawing_state::DrawingState;
use logic::screen::gui::MARGIN;
use logic::world::map::cell::{TextureIndex, TextureIndexTrait};

// #[derive(Clone)]
pub struct DrawerMacroquad {
    pub drawing: DrawingState,
    pub textures: Vec<Texture2D>,
    pub same_line: bool,
}

impl DrawerTrait for DrawerMacroquad {
    fn new(textures: Vec<Texture2D>) -> DrawerMacroquad {
        // let textures = load_tileset(tileset_path);
        // println!(
        //     "Loaded {} textures. The first one is {} by {} pixels",
        //     textures.len(),
        //     textures[0].width(),
        //     textures[0].height()
        // );
        let d = DrawerMacroquad {
            drawing: DrawingState::new(),
            textures,
            same_line: false,
        };
        // d._debug_draw_all_textures();
        d
    }

    fn set_textures(&mut self, textures: Vec<Texture2D>) {
        self.textures = textures;
    }

    fn take_textures(self: Box<Self>) -> Vec<Texture2D> {
        self.textures
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

    fn texture_size(&self, texture_index: &dyn TextureIndexTrait) -> Vec2 {
        let t = &self.textures[texture_index.get_index()];
        Vec2::new(t.width(), t.height())
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

    fn draw_rotated_texture(
        &self,
        texture: &dyn TextureIndexTrait,
        x: f32,
        y: f32,
        zoom: f32,
        color_mask: Color,
        rotation_radians: f32,
    ) {
        let texture = self.textures[texture.get_index()];
        macroquad_draw_texture_ex(
            texture,
            x,
            y,
            color_mask,
            DrawTextureParams {
                dest_size: Some(zoom_to_size(zoom, texture)),
                rotation: rotation_radians,
                ..Default::default()
            },
        );
    }

    fn draw_rectangle(&self, x: f32, y: f32, w: f32, h: f32, color: Color) {
        draw_rectangle(x, y, w, h, color);
    }
    fn draw_circle(&self, position: Vec2, radius: f32, color: Color) {
        draw_circle(position.x, position.y, radius, color);
    }
    fn draw_text(&self, text: &str, x: f32, y: f32, font_size: f32, color: Color) {
        draw_text(text, x, y, font_size, color);
    }
    fn measure_text(&mut self, text: &str, font_size: f32) -> Vec2 {
        self.ui_measure_text(text, font_size)
    }

    fn ui_run(&mut self, f: &mut dyn FnMut(&mut dyn DrawerTrait) -> ()) {
        f(self);
    }

    fn ui_draw(&mut self) {
        // macroquad automatically draws ui at the end of the frame
    }

    /// This grouping function does not support nested groups
    fn ui_group(
        &mut self,
        x: f32,
        y: f32,
        w: f32,
        h: f32,
        f: &mut dyn FnMut(&mut dyn DrawerTrait) -> (),
    ) -> Interaction {
        self.maybe_apply_same_line();
        let id = hash!(x.abs() as i32, y.abs() as i32);

        let window = widgets::Window::new(id, Vec2::new(x, y), Vec2::new(w, h))
            .titlebar(false)
            .movable(false);

        root_ui().same_line(0.0);
        let token = window.begin(&mut root_ui());
        f(self);
        token.end(&mut root_ui());
        get_interaction(x, y, w, h)
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
        self.maybe_apply_same_line();
        let id = hash!(title, x.abs() as i32, y.abs() as i32);
        let window = widgets::Window::new(id, Vec2::new(x, y), Vec2::new(w, h))
            .titlebar(true)
            .label(title)
            .movable(false);
        let token = window.begin(&mut root_ui());
        f(self);
        token.end(&mut root_ui());
        get_interaction(x, y, w, h)
    }

    fn ui_texture(&mut self, texture_index: TextureIndex) -> bool {
        self.maybe_apply_same_line();
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

    fn ui_texture_with_pos(
        &mut self,
        texture_index: &dyn TextureIndexTrait,
        x: f32,
        y: f32,
    ) -> bool {
        self.maybe_apply_same_line();
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

    fn ui_button(&mut self, text: &str) -> Interaction {
        self.maybe_apply_same_line();
        let clicked = root_ui().button(None, text);
        interaction_from_clicked(clicked)
    }

    fn ui_button_with_pos(&mut self, text: &str, x: f32, y: f32) -> Interaction {
        self.maybe_apply_same_line();
        let clicked = root_ui().button(Option::Some(Vec2::new(x, y)), text);
        interaction_from_clicked(clicked)
    }

    fn ui_checkbox(&mut self, checked: &mut bool, text: &str) {
        root_ui().checkbox(hash!(text), text, checked)
    }

    fn ui_slider(&mut self, x: f32, y: f32, label: &str, range: Range<f32>, number: &mut f32) {
        let id = hash!(x.abs() as i32, y.abs() as i32);
        widgets::Window::new(id, vec2(x, y), vec2(400., 60.))
            .titlebar(false)
            .ui(&mut *root_ui(), |ui| {
                ui.slider(id, label, range, number);
            });
    }

    fn ui_text(&mut self, text: &str) {
        self.maybe_apply_same_line();
        root_ui().label(None, text);
    }

    fn ui_measure_text(&mut self, text: &str, font_size: f32) -> Vec2 {
        let text_dimensions = measure_text(text, Option::None, font_size as u16, 1.0);
        Vec2::new(text_dimensions.width, text_dimensions.height)
    }

    fn ui_same_line(&mut self, f: &mut dyn FnMut(&mut dyn DrawerTrait) -> ()) {
        let previous_same_line = self.same_line;
        self.same_line = true;
        f(self);
        self.same_line = previous_same_line;
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
            title_height: font_size * 2.0,
            label_style,
            ..root_ui().default_skin()
        };
        root_ui().push_skin(&skin);
    }

    fn debug_ui(&mut self) {
        todo!()
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

impl DrawerMacroquad {
    fn get_textures(&self) -> &Vec<Texture2D> {
        &self.textures
    }

    pub fn get_texture_copy<T: Into<TextureIndex>>(&self, texture_index: T) -> Texture2D {
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

    pub fn maybe_apply_same_line(&self) {
        if self.same_line {
            root_ui().same_line(0.0);
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
        dest_size: Some(zoom_to_size(zoom, texture)),
        source: None,
        rotation: 0.0,
        flip_x: false,
        flip_y: false,
        pivot: None,
    }
}

fn zoom_to_size(zoom: f32, texture: Texture2D) -> Vec2 {
    Vec2::new(texture.height() * zoom, texture.width() * zoom)
}
