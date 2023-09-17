use crate::screen::drawer_trait::DrawerTrait;

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
}

impl Scene for IntroductionScene {
    fn frame(&mut self) -> State {
        State::ShouldFinish
    }
}
