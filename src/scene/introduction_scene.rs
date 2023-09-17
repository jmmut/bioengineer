mod fire_particles;

use crate::screen::drawer_trait::DrawerTrait;
use crate::screen::gui::BLACK;
use crate::world::map::cell::ExtraTextures;
use crate::Vec2;
use macroquad::prelude::{is_key_pressed, Color, KeyCode};
use std::f32::consts::PI;
use crate::scene::{Scene, State};
use crate::scene::introduction_scene::fire_particles::Particle;

pub struct IntroductionScene {
    pub drawer: Box<dyn DrawerTrait>,
    pub frame: i64,
    pub particles: Vec<Particle>,
    pub ship_pos: Vec2,
}

impl IntroductionScene {
    pub fn new(drawer: Box<dyn DrawerTrait>) -> Self {
        let (w, h) = (drawer.screen_width(), drawer.screen_height());
        Self {
            drawer,
            frame: 0,
            particles: Vec::new(),
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
            let rand = (((1+ self.frame % 17) * 38135 * (1+ self.frame / 4 % 5)) % 1000 - 500) as f32;
            self.particles.push(Particle {
                pos: self.ship_pos + Vec2::new(rand * 0.02, 0.0),
                direction: Vec2::new(0.0, -1.0) + Vec2::new(rand * 0.001, 0.0)
            });
            let yellow = Color::new(0.8, 0.9, 0.5, 0.75);
            let mut to_remove = Vec::new();
            for (i, particle) in &mut self.particles.iter_mut().enumerate() {
                particle.pos += particle.direction;
                if particle.pos.y < 100.0 {
                    to_remove.push(i);
                }
                self.drawer
                    .draw_rectangle(particle.pos.x, particle.pos.y, 5.0, 5.0, yellow);
            }
            for i in to_remove.iter().rev() {
                self.particles.swap_remove(*i);
            }
            let color_mask = Color::new(1.0, 1.0, 1.0, 1.0);
            let texture_size = self.drawer.texture_size(&ExtraTextures::Ship);
            self.drawer.draw_rotated_texture(
                &ExtraTextures::Ship,
                self.drawer.screen_width() * 0.5 - texture_size.x * 0.5,
                self.drawer.screen_height() * 0.5 - texture_size.y * 0.5,
                1.0,
                color_mask,
                PI,
            );
            State::ShouldContinue
        }
    }
}
