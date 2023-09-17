use crate::screen::drawer_trait::DrawerTrait;
use crate::screen::gui::BLACK;
use crate::world::map::cell::ExtraTextures;
use crate::Vec2;
use macroquad::prelude::{is_key_pressed, Color, KeyCode};
use std::f32::consts::PI;

pub trait Scene {
    fn frame(&mut self) -> State;
}

#[derive(Eq, PartialEq)]
pub enum State {
    ShouldContinue,
    ShouldFinish,
}

pub struct IntroductionScene {
    pub drawer: Box<dyn DrawerTrait>,
    pub frame: i64,
    pub particles: Vec<Vec2>,
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
            self.particles.push(self.ship_pos);
            let yellow = Color::new(0.8, 0.9, 0.5, 0.75);
            let mut to_remove = Vec::new();
            for (i, particle) in &mut self.particles.iter_mut().enumerate() {
                particle.y -= 1.0;
                if particle.y < 100.0 {
                    to_remove.push(i);
                }
                self.drawer
                    .draw_rectangle(particle.x, particle.y, 5.0, 5.0, yellow);
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
