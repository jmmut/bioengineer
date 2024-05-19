pub mod introduction_scene;
pub mod main_scene;

pub trait Scene {
    fn frame(&mut self) -> GameLoopState;
}

#[derive(Eq, PartialEq, Copy, Clone)]
#[repr(C)]
pub enum GameLoopState {
    ShouldContinue,
    ShouldFinish,
    ShouldRepositionCamera, // hmm this feels out of place
}

impl GameLoopState {
    pub fn should_continue(&self) -> bool {
        *self != GameLoopState::ShouldFinish
    }
}
