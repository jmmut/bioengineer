pub mod introduction_scene;
pub mod main_scene;

pub trait Scene {
    fn frame(&mut self) -> State;
}

#[derive(Eq, PartialEq)]
#[repr(C)]
pub enum State {
    ShouldContinue,
    ShouldFinish,
}
