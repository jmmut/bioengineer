mod fire_particles;

use crate::scene::introduction_scene::fire_particles::Particle;
use crate::scene::{Scene, State};
use crate::screen::drawer_trait::DrawerTrait;
use crate::screen::gui::BLACK;
use crate::world::map::cell::ExtraTextures;
use crate::Vec2;
use macroquad::prelude::{draw_circle, info, is_key_pressed, Color, KeyCode};
use std::f32::consts::PI;

const WHITE: Color = Color::new(1.0, 1.0, 1.0, 1.0);

pub struct IntroductionScene {
    pub drawer: Box<dyn DrawerTrait>,
    pub frame: i64,
    pub fire: Vec<Particle>,
    pub stars: Vec<Particle>,
    pub stars_opacity: f32,
    pub ship_pos: Vec2,
}

const STARS_SPEED: f32 = 0.25;

impl IntroductionScene {
    pub fn new(drawer: Box<dyn DrawerTrait>) -> Self {
        let w = drawer.screen_width();
        let height = drawer.screen_height();
        let width = drawer.screen_width();
        let mut stars = Vec::new();
        for i in 0..100 {
            let rand = next_rand(i);
            let rand2 = next_rand((rand * 2000.0) as i64);
            let rand3 = next_rand((rand * rand2 * 1000.0) as i64);
            stars.push(Particle {
                pos: Vec2::new(rand * width, rand2 * height),
                direction: Vec2::new(0.0, -STARS_SPEED - rand3 * 0.0625),
                opacity: 1.0,
            });
        }
        let texture_size = drawer.texture_size(&ExtraTextures::Ship) * ZOOM;
        Self {
            drawer,
            frame: 0,
            fire: Vec::new(),
            stars,
            stars_opacity: 0.0,
            ship_pos: Vec2::new(w * 0.5, -texture_size.y),
        }
    }
}

const ZOOM: f32 = 2.0;

impl Scene for IntroductionScene {
    fn frame(&mut self) -> State {
        self.frame = (self.frame + 1) % 100000000;
        if is_key_pressed(KeyCode::Escape) {
            State::ShouldFinish
        } else {
            self.drawer.clear_background(BLACK);
            let height = self.drawer.screen_height();
            let width = self.drawer.screen_width();

            if self.ship_pos.y < height * 0.5 {
                self.ship_pos.y += 2.0;
            }

            let rand = next_rand(self.frame);

            if self.frame % 20 == 0 {
                let rand2 = next_rand((self.frame as f32 * rand) as i64);
                self.stars.push(Particle {
                    pos: Vec2::new(rand * width, height),
                    direction: Vec2::new(0.0, -STARS_SPEED - rand2 * 0.0625),
                    opacity: 1.0,
                });
            }
            self.stars_opacity = (self.stars_opacity + 0.002).min(1.0);
            let mut to_remove = Vec::new();
            for (i, particle) in &mut self.stars.iter_mut().enumerate() {
                particle.pos += particle.direction;
                if particle.pos.x < 0.0 {
                    to_remove.push(i);
                }
                let rand2 = next_rand(i as i64 + (rand * 1000.0) as i64);
                // particle.opacity
                let mut white = Color::new(
                    1.0,
                    1.0,
                    1.0,
                    self.stars_opacity - self.stars_opacity * rand2 * 0.75,
                );
                // draw_circle(particle.pos.x, particle.pos.y, 1.5, white);

                self.drawer.draw_rectangle(
                    particle.pos.x - 1.5,
                    particle.pos.y - 1.5,
                    3.0,
                    3.0,
                    white,
                );
                white.a *= 0.03;
                // if particle.pos.x < width * 0.5 {
                draw_circle(particle.pos.x, particle.pos.y, 8.0, white);
                // }
                // self.drawer
                //     .draw_rectangle(particle.pos.x - 5.0, particle.pos.y-5.0, 13.0, 13.0, white);
            }
            for i in to_remove.iter().rev() {
                self.stars.swap_remove(*i);
            }
            let pos_x = if self.frame % 2 == 0 {
                ((rand - 0.5) * 2.0 + 5.0) * ZOOM
            } else {
                ((rand - 0.5) * 2.0 - 12.5) * ZOOM
            };
            self.fire.push(Particle {
                pos: self.ship_pos + Vec2::new(pos_x, 5.0),
                direction: Vec2::new(0.0, -3.0) + Vec2::new((rand - 0.5) * 2.0, 0.0),
                opacity: 1.0,
            });
            let mut to_remove = Vec::new();
            for (i, particle) in &mut self.fire.iter_mut().enumerate() {
                particle.pos += particle.direction;
                let particle_ship_diff = self.ship_pos - particle.pos;
                particle.opacity = 1.0 - particle_ship_diff.length() / 200.0;
                if particle.opacity < 0.0 {
                    to_remove.push(i);
                }

                let yellow = Color::new(0.8, 0.9, 0.5, 0.75 * particle.opacity);
                self.drawer
                    .draw_rectangle(particle.pos.x, particle.pos.y, 5.0, 5.0, yellow);
            }
            for i in to_remove.iter().rev() {
                self.fire.swap_remove(*i);
            }
            let texture_size = self.drawer.texture_size(&ExtraTextures::Ship) * ZOOM;
            self.drawer.draw_rotated_texture(
                &ExtraTextures::Ship,
                self.ship_pos.x - texture_size.x * 0.5,
                self.ship_pos.y - texture_size.y * 0.5,
                ZOOM,
                WHITE,
                PI,
            );
            State::ShouldContinue
        }
    }
}

/// Given a seed, returns a float in the range of [0, 1]
fn next_rand(seed: i64) -> f32 {
    ((((1 + seed % 101) * 38135) % 101 * 31 * (1 + seed / 4 % 37)) % 1000) as f32 / 1000.0
}
