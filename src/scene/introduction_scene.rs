mod fire_particles;

use crate::scene::introduction_scene::fire_particles::Particle;
use crate::scene::{Scene, State};
use crate::screen::drawer_trait::DrawerTrait;
use crate::screen::gui::BLACK;
use crate::world::map::cell::ExtraTextures;
use crate::Vec2;
use macroquad::prelude::{info, is_key_pressed, Color, KeyCode};
use std::f32::consts::PI;

pub struct IntroductionScene {
    pub drawer: Box<dyn DrawerTrait>,
    pub frame: i64,
    pub fire: Vec<Particle>,
    pub stars: Vec<Particle>,
    pub ship_pos: Vec2,
}

impl IntroductionScene {
    pub fn new(drawer: Box<dyn DrawerTrait>) -> Self {
        let (w, h) = (drawer.screen_width(), drawer.screen_height());
        Self {
            drawer,
            frame: 0,
            fire: Vec::new(),
            stars: Vec::new(),
            ship_pos: Vec2::new(w * 0.5, h * 0.5),
        }
    }
}

impl Scene for IntroductionScene {
    fn frame(&mut self) -> State {
        self.frame += 1;
        if is_key_pressed(KeyCode::Escape) {
            State::ShouldFinish
        } else {
            self.drawer.clear_background(BLACK);
            let rand =
                (((1 + self.frame % 17) * 38135 * (1 + self.frame / 4 % 5)) % 1000 - 500) as f32;
            let zoom = 2.0;
            self.fire.push(Particle {
                pos: self.ship_pos + Vec2::new(rand * 0.025 * zoom, 5.0),
                direction: Vec2::new(0.0, -3.0) + Vec2::new(rand * 0.002, 0.0),
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
            let color_mask = Color::new(1.0, 1.0, 1.0, 1.0);
            let texture_size = self.drawer.texture_size(&ExtraTextures::Ship) * zoom;
            self.drawer.draw_rotated_texture(
                &ExtraTextures::Ship,
                self.drawer.screen_width() * 0.5 - texture_size.x * 0.5,
                self.drawer.screen_height() * 0.5 - texture_size.y * 0.5,
                zoom,
                color_mask,
                PI,
            );
            State::ShouldContinue
        }
    }
}
