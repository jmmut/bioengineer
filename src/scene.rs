use crate::screen::drawer_trait::DrawerTrait;
use crate::screen::gui::BLACK;
use crate::world::map::cell::ExtraTextures;
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
}

impl IntroductionScene {
    pub fn new(drawer: Box<dyn DrawerTrait>) -> Self {
        Self { drawer, frame: 0 }
    }
}

impl Scene for IntroductionScene {
    fn frame(&mut self) -> State {
        self.frame += 1;
        if is_key_pressed(KeyCode::Escape) {
            State::ShouldFinish
        } else {
            self.drawer.clear_background(BLACK);
            let color_mask = Color::new(1.0, 1.0, 1.0, 1.0);
            self.drawer.draw_rotated_texture(
                &ExtraTextures::Ship,
                self.drawer.screen_width() * 0.5,
                self.drawer.screen_height() * 0.5,
                1.0,
                color_mask,
                PI,
            );
            State::ShouldContinue
        }
    }
}
