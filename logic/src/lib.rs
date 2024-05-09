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

pub fn frame(scene_wrapper: &mut Box<SceneState>) -> GameLoopState {
    scene_wrapper.frame()
}

/// scene_wrapper has to be a box because a Rust enum can not be passed through a Foreign Function
/// Interface (aka, C dynamic library API)
#[no_mangle]
pub extern "C" fn hot_reload_draw_frame(scene_wrapper: &mut Box<SceneState>) -> GameLoopState {
    frame(scene_wrapper)
}

impl Scene for SceneState {
    fn frame(&mut self) -> GameLoopState {
        match self {
            SceneState::Introduction(intro_scene) => intro_scene.frame(),
            SceneState::Main(main_scene) => main_scene.frame(),
        }
    }
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
