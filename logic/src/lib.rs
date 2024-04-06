//! The logic crate contains code about the game without including code from libraries (like macroquad).
//!
//! The purpose of splitting this crate in the workspace is to be able to build a small-ish dynamic
//! library that can be reloaded at runtime. See game/src/bin/hot_reload_bioengineer.rs.
//!
//! Another benefit is that this structure requires taking interfaces for all functions with
//! external effects, so it's possible to provide mocked implementations of them for integration
//! tests. See [crate::world::gameplay_tests].

use mq_basics::Texture2D;
use crate::scene::introduction_scene::{IntroductionScene, IntroductionSceneState};
use crate::scene::main_scene::MainScene;
use crate::scene::{Scene, State};

pub mod common {
    pub mod profiling;
    pub mod trunc;
}
pub mod screen;
pub mod world;
pub mod scene;

pub mod external {
    pub mod assets_macroquad;
    // pub mod backends;
    // pub mod drawer_egui_macroquad;
    // pub mod drawer_macroquad;
    // pub mod input_macroquad;
    // pub mod main_input_macroquad;
}

pub enum SceneState {
    Introduction(IntroductionSceneState),
    Main(MainScene),
}
impl SceneState {
    pub fn take_textures(self) -> Vec<Texture2D> {
        match self {
            SceneState::Introduction(state) => state.take_textures(),
            SceneState::Main(state) => state.screen.drawer.take_textures(),
        }
    }
}

pub fn frame(scene_wrapper: &mut Box<Option<SceneState>>) -> State {
    let wrapper = scene_wrapper.take().unwrap();
    match wrapper {
        SceneState::Introduction(scene_state) => {
            let mut scene = IntroductionScene { state: scene_state };
            let output_state = scene.frame();
            scene_wrapper.replace(SceneState::Introduction(scene.state));
            output_state
        }
        SceneState::Main(mut main_scene) => {
            let output_state = main_scene.frame();
            scene_wrapper.replace(SceneState::Main(main_scene));
            output_state
        }
    }
}

#[no_mangle]
pub extern "C" fn hot_reload_draw_frame(scene_wrapper: &mut Box<Option<SceneState>>) -> State {
    frame(scene_wrapper)
}
