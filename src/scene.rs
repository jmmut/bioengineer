use crate::screen::drawer_trait::DrawerTrait;
use crate::screen::GREY;
use crate::screen::gui::BLACK;

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
        if self.frame < 200 {
            self.drawer.clear_background(GREY);
            let text = "Loading...";
            let font_size = 32.0;
            let text_size = self.drawer.measure_text(text, font_size);
            self.drawer.draw_text(
                text,
                (self.drawer.screen_width() * 0.5 - text_size.x * 0.5).round(),
                (self.drawer.screen_height() * 0.5 + text_size.y * 0.5).round(),
                font_size,
                BLACK,
            );
            State::ShouldContinue
        } else {
            State::ShouldFinish
        }
    }
}
