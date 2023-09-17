pub mod introduction_scene;

pub trait Scene {
    fn frame(&mut self) -> State;
}

#[derive(Eq, PartialEq)]
pub enum State {
    ShouldContinue,
    ShouldFinish,
}
