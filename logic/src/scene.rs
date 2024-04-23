pub mod introduction_scene;
pub mod main_scene;

pub trait Scene {
    fn frame(&mut self) -> GameLoopState;
}

#[derive(Eq, PartialEq)]
#[repr(C)]
pub enum GameLoopState {
    ShouldContinue,
    ShouldFinish,
}
