//! The logic crate contains code about the game without including code from libraries (like macroquad).
//!
//! The purpose of splitting this crate in the workspace is to be able to build a small-ish dynamic
//! library that can be reloaded at runtime. See game/src/bin/hot_reload_bioengineer.rs.
//!
//! Another benefit is that this structure requires taking interfaces for all functions with
//! external effects, so it's possible to provide mocked implementations of them for integration
//! tests. See [crate::world::gameplay_tests].

use crate::scene::introduction_scene::IntroductionScene;
use crate::scene::main_scene::MainScene;
use crate::scene::{GameLoopState, Scene};
use mq_basics::Texture2D;

pub mod common {
    pub mod profiling;
    pub mod trunc;
}
pub mod scene;
pub mod screen;
pub mod world;

pub enum SceneState {
    Introduction(IntroductionScene),
    Main(MainScene),
}
impl SceneState {
    pub fn take_textures(self) -> Vec<Texture2D> {
        match self {
            SceneState::Introduction(state) => state.take_textures(),
            SceneState::Main(state) => state.screen.drawer.take_textures(),
        }
    }
    pub fn set_textures(&mut self, textures: Vec<Texture2D>) {
        match self {
            SceneState::Introduction(state) => state.set_textures(textures),
            SceneState::Main(state) => state.screen.drawer.set_textures(textures),
        }
    }
}

pub fn frame(scene_wrapper: &mut Box<SceneState>) -> GameLoopState {
    let wrapper = scene_wrapper.as_mut();
    match wrapper {
        SceneState::Introduction(intro_scene) => {
            let output_state = intro_scene.frame();
            output_state
        }
        SceneState::Main(main_scene) => {
            let output_state = main_scene.frame();
            output_state
        }
    }
}

#[no_mangle]
pub extern "C" fn hot_reload_draw_frame(scene_wrapper: &mut Box<SceneState>) -> GameLoopState {
    frame(scene_wrapper)
}
