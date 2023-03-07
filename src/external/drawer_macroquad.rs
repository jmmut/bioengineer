use macroquad::color::Color;
use macroquad::hash;
use macroquad::input::{is_mouse_button_pressed, mouse_position, MouseButton};
use macroquad::math::{Rect, RectOffset, Vec2};
use macroquad::prelude::Texture2D;
use macroquad::shapes::draw_rectangle;
use macroquad::text::{draw_text, measure_text};
use macroquad::texture::draw_texture as macroquad_draw_texture;
use macroquad::ui::widgets::Texture;
use macroquad::ui::{root_ui, widgets, Skin};
use macroquad::window::{clear_background, screen_height, screen_width};

use crate::screen::assets;
use crate::screen::assets::{PIXELS_PER_TILE_HEIGHT, PIXELS_PER_TILE_WIDTH};
use crate::screen::drawer_trait::{DrawerTrait, Interaction};
use crate::screen::drawing_state::DrawingState;
use crate::screen::gui::{FONT_SIZE, MARGIN};
use crate::world::map::cell::TextureIndex;

pub struct DrawerMacroquad {
    pub drawing: DrawingState,
    pub textures: Vec<Texture2D>,
}

impl DrawerTrait for DrawerMacroquad {
    fn new(textures: Vec<Texture2D>) -> DrawerMacroquad {
        // let textures = load_tileset(tileset_path);
        println!(
            "Loaded {} textures. The first one is {} by {} pixels",
            textures.len(),
            textures[0].width(),
            textures[0].height()
        );
        let d = DrawerMacroquad {
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
    fn draw_texture<T>(&self, texture_index: T, x: f32, y: f32)
    where
        T: Into<TextureIndex>,
    {
        self.draw_transparent_texture(texture_index, x, y, 1.0);
    }

    fn draw_transparent_texture<T>(&self, texture: T, x: f32, y: f32, opacity_coef: f32)
    where
        T: Into<TextureIndex>,
    {
        let mask_color = Color::new(1.0, 1.0, 1.0, opacity_coef);
        macroquad_draw_texture(self.textures[texture.into().get_index()], x, y, mask_color);
    }
    fn draw_colored_texture<T>(&self, texture: T, x: f32, y: f32, color_mask: Color)
    where
        T: Into<TextureIndex>,
    {
        macroquad_draw_texture(self.textures[texture.into().get_index()], x, y, color_mask);
    }
    fn draw_rectangle(&self, x: f32, y: f32, w: f32, h: f32, color: Color) {
        draw_rectangle(x, y, w, h, color);
    }
    fn draw_text(&self, text: &str, x: f32, y: f32, font_size: f32, color: Color) {
        draw_text(text, x, y, font_size, color);
    }

    /// This grouping function does not support nested groups
    fn ui_group<F: FnOnce()>(&self, x: f32, y: f32, w: f32, h: f32, f: F) -> Interaction {
        let id = hash!(x.abs() as i32, y.abs() as i32);
        let window = widgets::Window::new(id, Vec2::new(x, y), Vec2::new(w, h))
            .titlebar(false)
            .movable(false);
        let token = window.begin(&mut root_ui());
        f();
        token.end(&mut root_ui());
        let mouse_clicked = is_mouse_button_pressed(MouseButton::Left);
        if mouse_clicked {
            return Interaction::Clicked;
        } else {
            let (mouse_x, mouse_y) = mouse_position();
            let group_rect = Rect { x, y, w, h };
            if group_rect.contains(Vec2::new(mouse_x, mouse_y)) {
                root_ui().focus_window(id);
                return Interaction::Hovered;
            }
        }
        Interaction::None
    }

    fn ui_named_group<F: FnOnce()>(
        &self,
        title: &str,
        x: f32,
        y: f32,
        w: f32,
        h: f32,
        f: F,
    ) -> Interaction {
        let id = hash!(title, x.abs() as i32, y.abs() as i32);
        let window = widgets::Window::new(id, Vec2::new(x, y), Vec2::new(w, h))
            .titlebar(true)
            .label(title)
            .movable(false);
        let token = window.begin(&mut root_ui());
        f();
        token.end(&mut root_ui());
        let mouse_clicked = is_mouse_button_pressed(MouseButton::Left);
        if mouse_clicked {
            return Interaction::Clicked;
        } else {
            let (mouse_x, mouse_y) = mouse_position();
            let group_rect = Rect { x, y, w, h };
            if group_rect.contains(Vec2::new(mouse_x, mouse_y)) {
                root_ui().focus_window(id);
                return Interaction::Hovered;
            }
        }
        Interaction::None
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

    fn ui_texture_with_pos<T>(&self, texture_index: T, x: f32, y: f32) -> bool
    where
        T: Into<TextureIndex>,
    {
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

impl DrawerMacroquad {
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
