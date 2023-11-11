use juquad::texture_loader::TextureLoader;
use juquad::widgets::anchor::Anchor;
use juquad::widgets::button::{Button, InteractionStyle, Style};
use std::f32::consts::PI;

use crate::external::assets_macroquad::split_tileset;
use macroquad::prelude::{get_fps, Color, Image, KeyCode, MouseButton, Texture2D, DARKGRAY};

use crate::scene::introduction_scene::fire_particles::Particle;
use crate::scene::{Scene, State};
use crate::screen::drawer_trait::DrawerTrait;
use crate::screen::gui::{
    BACKGROUND_UI_COLOR_BUTTON, BACKGROUND_UI_COLOR_BUTTON_CLICKED,
    BACKGROUND_UI_COLOR_BUTTON_HOVERED, BLACK, BUTTON_TEXT_COLOR, FONT_SIZE,
};
use crate::screen::input_trait::InputTrait;
use crate::world::map::cell::ExtraTextures;
use crate::Vec2;

mod fire_particles;

const WHITE: Color = Color::new(1.0, 1.0, 1.0, 1.0);
const MAX_TTL: f32 = 60.0;
const STARS_SPEED: f32 = 0.25;
const ZOOM: f32 = 2.0;
const STYLE: Style = Style {
    bg_color: InteractionStyle {
        at_rest: BACKGROUND_UI_COLOR_BUTTON,
        hovered: BACKGROUND_UI_COLOR_BUTTON_HOVERED,
        pressed: BACKGROUND_UI_COLOR_BUTTON_CLICKED,
    },
    text_color: InteractionStyle {
        at_rest: BUTTON_TEXT_COLOR,
        hovered: BUTTON_TEXT_COLOR,
        pressed: BUTTON_TEXT_COLOR,
    },
    border_color: InteractionStyle {
        at_rest: DARKGRAY,
        hovered: Color::new(0.88, 0.88, 0.88, 1.00),
        pressed: DARKGRAY,
    },
};

pub struct IntroductionSceneState {
    pub drawer: Option<Box<dyn DrawerTrait>>,
    pub input: Option<Box<dyn InputTrait>>,
    pub frame: i64,
    pub fire: Vec<Particle>,
    pub stars: Vec<Particle>,
    pub stars_opacity: f32,
    pub ship_pos: Vec2,
    pub show_new_game_button: bool,
    pub loader: TextureLoader<Image>,
    pub textures_ready: bool,
}

pub struct IntroductionScene {
    pub state: IntroductionSceneState,
}

impl IntroductionSceneState {
    pub fn new(
        drawer: Box<dyn DrawerTrait>,
        input: Box<dyn InputTrait>,
        loader: TextureLoader<Image>,
    ) -> Self {
        let width = drawer.screen_width();
        let height = drawer.screen_height();
        let mut stars = Vec::new();
        for i in 0..100 {
            let rand = next_rand(i);
            let rand2 = next_rand((rand * 2000.0) as i64);
            let rand3 = next_rand((rand * rand2 * 1000.0) as i64);
            stars.push(Particle {
                pos: Vec2::new(rand * width, rand2 * height),
                direction: Vec2::new(0.0, -STARS_SPEED - rand3 * 0.0625),
                opacity: 1.0,
                time_to_live: -1,
                user_float: next_rand((rand * rand2 * rand3 * 10000.0) as i64) as f64,
                user_int: 0,
            });
        }
        // let texture_size = drawer.texture_size(&ExtraTextures::Ship) * ZOOM; // TODO: move this after loading textures
        Self {
            drawer: Some(drawer),
            input: Some(input),
            frame: 0,
            fire: Vec::new(),
            stars,
            stars_opacity: 0.0,
            ship_pos: Vec2::new(width * 0.5, 0.0),
            show_new_game_button: false,
            loader,
            textures_ready: false,
        }
    }
    fn reset(&mut self) {
        let mut fake_loader = TextureLoader::new_from_image(&[]);
        std::mem::swap(&mut fake_loader, &mut self.loader);
        *self = Self::new(
            self.drawer.take().unwrap(),
            self.input.take().unwrap(),
            fake_loader,
        )
    }

    fn try_load_textures(&mut self) {
        if !self.textures_ready {
            let loaded = self.loader.get_textures();
            match loaded {
                Ok(Some(atlas)) => {
                    println!("loaded!");
                    let textures = split_tileset(&atlas[0]);
                    self.drawer.as_mut().unwrap().set_textures(textures);
                    self.textures_ready = true;
                    let texture_size = self
                        .drawer
                        .as_ref()
                        .unwrap()
                        .texture_size(&ExtraTextures::Ship)
                        * ZOOM;
                    self.ship_pos = Vec2::new(
                        self.drawer.as_ref().unwrap().screen_width() * 0.5,
                        -texture_size.y,
                    )
                }
                Ok(None) => {}
                Err(e) => {
                    panic!("{}", e);
                }
            }
        }
    }
    pub fn take_textures(self) -> Vec<Texture2D> {
        self.drawer.unwrap().take_textures()
    }
}

impl Scene for IntroductionScene {
    fn frame(&mut self) -> State {
        self.state.frame = (self.state.frame + 1) % 100000000;
        let height = self.state.drawer.as_mut().unwrap().screen_height();
        let width = self.state.drawer.as_mut().unwrap().screen_width();

        self.state.try_load_textures();

        let (buttons, new_game_clicked) = self.ui_interact(height, width);

        if new_game_clicked {
            State::ShouldFinish
        } else {
            self.render(height, width, &buttons);
            State::ShouldContinue
        }
    }
}

impl IntroductionScene {
    fn ui_interact(&mut self, height: f32, width: f32) -> (Vec<Button>, bool) {
        if self.input().is_key_pressed(KeyCode::R) {
            self.state.reset();
        }
        if self.input().is_key_pressed(KeyCode::F3) {
            println!("FPS: {}", get_fps())
        }
        let mut buttons = Vec::new();
        if self.state.ship_pos.y < height * 0.5 {
            self.state.ship_pos.y += 2.0;
        } else {
            self.state.show_new_game_button = true;
        }
        let new_game_clicked = if self.state.show_new_game_button {
            // juquad::Button is incompatible with hot reloading. Will need drawer_juquad :DrawerTrait
            let mut button = Button::new(
                "New Game",
                Anchor::center(0.5 * width, 0.8 * height),
                FONT_SIZE,
            );
            let interaction = button.interact().is_clicked();
            buttons.push(button);
            interaction
                || self.is_any_key_pressed(&[KeyCode::Space, KeyCode::Enter, KeyCode::KpEnter])
        } else {
            false
        };
        if self.input().is_mouse_button_pressed(MouseButton::Left)
            || self.is_any_key_pressed(&[KeyCode::Space, KeyCode::Enter, KeyCode::KpEnter])
        {
            self.state.show_new_game_button = true;
        }

        if self.input().is_key_down(KeyCode::Right) {
            if self.state.ship_pos.x < width * 0.8 {
                self.state.ship_pos.x += 2.0;
            }
        }
        if self.input().is_key_down(KeyCode::Left) {
            if self.state.ship_pos.x > width * 0.2 {
                self.state.ship_pos.x -= 2.0;
            }
        }
        (
            buttons,
            new_game_clicked || self.input().is_key_pressed(KeyCode::Escape),
        )
    }

    fn render(&mut self, height: f32, width: f32, buttons: &Vec<Button>) {
        self.state.drawer.as_mut().unwrap().clear_background(BLACK);

        let rand = next_rand(self.state.frame);

        self.draw_stars(height, width, rand);

        if self.state.textures_ready {
            self.draw_fire(rand);

            let drawer = self.state.drawer.as_mut().unwrap();
            let texture_size = drawer.texture_size(&ExtraTextures::Ship) * ZOOM;
            drawer.draw_rotated_texture(
                &ExtraTextures::Ship,
                self.state.ship_pos.x - texture_size.x * 0.5,
                self.state.ship_pos.y - texture_size.y * 0.5,
                ZOOM,
                WHITE,
                PI,
            );
            for button in buttons {
                button.render(&STYLE);
            }
        }
    }

    fn draw_stars(&mut self, height: f32, width: f32, rand: f32) {
        if self.state.frame % 20 == 0 {
            let rand2 = next_rand((self.state.frame as f32 * rand) as i64);
            let rand3 = next_rand((rand * rand2 * 1000.0) as i64);
            self.state.stars.push(Particle {
                pos: Vec2::new(rand * width, height),
                direction: Vec2::new(0.0, -STARS_SPEED - rand2 * 0.0625),
                opacity: 1.0,
                time_to_live: -1,
                user_float: next_rand((rand * rand2 * rand3 * 10000.0) as i64) as f64,
                user_int: 0,
            });
        }
        self.state.stars_opacity = (self.state.stars_opacity + 0.02).min(1.0);
        let mut to_remove = Vec::new();
        for (i, particle) in &mut self.state.stars.iter_mut().enumerate() {
            particle.pos += particle.direction;
            if particle.pos.x < 0.0 {
                to_remove.push(i);
            }
            let rand2 = next_rand(i as i64 + (rand * 1000.0) as i64);
            let (orange, blue) = if particle.user_float >= 0.5 {
                (particle.user_float as f32, 0.0)
            } else {
                (0.0, particle.user_float as f32)
            };
            // particle.opacity
            let mut white = Color::new(
                1.0 - blue * 0.35,
                1.0 - next_rand((particle.user_float * 1000.0) as i64) * 0.0625,
                1.0 - orange * 0.35,
                self.state.stars_opacity - self.state.stars_opacity * rand2 * 0.75,
            );

            if white.g < 0.5 && white.b < 0.5 {
                println!("wut");
            }
            // draw_circle(particle.pos.x, particle.pos.y, 1.5, white);

            self.state.drawer.as_mut().unwrap().draw_rectangle(
                particle.pos.x - 1.5,
                particle.pos.y - 1.5,
                3.0,
                3.0,
                white,
            );
            white.a *= 0.03;
            self.state
                .drawer
                .as_mut()
                .unwrap()
                .draw_circle(particle.pos, 8.0, white);
            // TODO: draw black circles to simulate a brightness cross
        }

        for i in to_remove.iter().rev() {
            self.state.stars.swap_remove(*i);
        }
    }

    fn draw_fire(&mut self, rand: f32) {
        let exhaust_x = if self.state.frame % 2 == 0 {
            7.5
            // + 25.0
        } else {
            -10.5
            // + 30.0
        };
        let exhaust_y = if self.state.frame % 2 == 0 { -2.0 } else { 0.0 };
        let side = 1.5
            * if self.state.frame / 2 % 2 == 0 {
                -1.0
            } else {
                1.0
            };
        let pos_x = (exhaust_x + side) * ZOOM;
        let centered_rand = rand - 0.5;
        if self.state.frame % 8 >= 0 {
            self.state.fire.push(Particle {
                pos: Vec2::new(pos_x, exhaust_y) + Vec2::new(centered_rand, centered_rand),
                // direction: Vec2::new(0.0, -3.0) + Vec2::new((rand - 0.5) * 2.0, 0.0),
                direction: Vec2::new(0.0, -3.0) + Vec2::new(0.125 * centered_rand, -rand * 0.5),
                opacity: 1.0,
                time_to_live: (rand * MAX_TTL * 0.5 + MAX_TTL * 0.5) as i64,
                user_float: 0.0,
                user_int: 0,
            });
        }
        let mut to_remove = Vec::new();
        let size = Vec2::new(20.0, 20.0);
        for (i, particle) in &mut self.state.fire.iter_mut().enumerate() {
            particle.pos += particle.direction;
            // let particle_ship_diff = self.state.ship_pos - particle.pos;
            // let age = MAX_TTL - particle.time_to_live as f32;
            // particle.opacity = 1.0 - (age * age) / (max_ttl * max_ttl);
            particle.time_to_live -= 1;
            if particle.time_to_live <= 0 {
                to_remove.push(i);
            }

            let yellow = fire_color(&particle, rand);
            self.state.drawer.as_mut().unwrap().draw_rectangle(
                self.state.ship_pos.x + particle.pos.x - size.x * 0.5,
                self.state.ship_pos.y + particle.pos.y - size.y,
                size.x,
                size.y,
                yellow,
            );
        }
        for i in to_remove.iter().rev() {
            self.state.fire.swap_remove(*i);
        }
        // self.debug_render_particles(size);
    }

    #[allow(unused)]
    fn debug_render_particles(&self, size: Vec2) {
        for i in 0..60 {
            let particle = Particle {
                pos: Vec2::new((20 * i) as f32 + 20.0, 100.0),
                direction: Vec2::new(0.0, 0.0),
                opacity: 1.0,
                time_to_live: i,
                user_float: 0.0,
                user_int: 0,
            };
            let color = fire_color(&particle, 0.0);
            self.state.drawer.as_ref().unwrap().draw_rectangle(
                particle.pos.x - size.x * 0.5,
                particle.pos.y - size.y,
                size.x,
                size.y,
                color,
            );
        }
    }

    fn input(&self) -> &dyn InputTrait {
        self.state.input.as_ref().unwrap().as_ref()
    }

    fn is_any_key_pressed(&self, keys: &[KeyCode]) -> bool {
        for key in keys {
            if self.input().is_key_pressed(*key) {
                return true;
            }
        }
        return false;
    }
}

fn fire_color(particle: &Particle, _rand: f32) -> Color {
    let rand = 0.0;
    let ttl_coef = (particle.time_to_live as f32 / (MAX_TTL - 1.0) * 4.0).min(1.0);
    let big_small_small = ttl_coef * ttl_coef;
    let _small_big_big = 1.0 - big_small_small;
    let small_small_big = (1.0 - ttl_coef) * (1.0 - ttl_coef);
    let big_big_small = 1.0 - small_small_big;
    let _big_small_big = ((big_small_small + small_small_big) * 1.0).min(1.0);
    let _medium_small_big = (big_small_small * 0.4 + 0.6 * small_small_big).min(1.0);
    let big_small_medium = (big_small_small * 0.6 + 0.4 * small_small_big).min(1.0);
    Color::new(
        0.8 + rand * 0.1 + big_big_small * 0.2,
        0.5 + rand * 0.1 + big_small_small * 0.5,
        0.25 + rand * 0.1 + (big_small_medium * 0.75),
        0.75 * particle.opacity * ttl_coef,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fire_color() {
        for i in (0..60).rev() {
            let particle = Particle {
                pos: Vec2::new((20 * i) as f32 + 20.0, 100.0),
                direction: Vec2::new(0.0, 0.0),
                opacity: 1.0,
                time_to_live: i,
                user_float: 0.0,
                user_int: 0,
            };
            let color = fire_color(&particle, 0.0);

            println!("{:?}", color);
        }
    }
}

/// Given a seed, returns a float in the range of [0, 1)
fn next_rand(seed: i64) -> f32 {
    ((((1 + seed % 101) * 38135) % 101 * 31 * (1 + seed / 4 % 37)) % 1000) as f32 / 1000.0
}
