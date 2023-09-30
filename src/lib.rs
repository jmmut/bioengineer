pub mod common {
    pub mod cli;
    pub mod profiling;
    pub mod trunc;
}
pub mod scene;
pub mod screen;
pub mod world;

pub mod external {
    pub mod assets_macroquad;
    pub mod backends;
    pub mod drawer_egui_macroquad;
    pub mod drawer_macroquad;
    pub mod input_macroquad;
}

use macroquad::color::Color;
use macroquad::math::{IVec2, IVec3, Rect, Vec2, Vec3};
use macroquad::miniquad::date::now;
use macroquad::texture::{Image, Texture2D};

use crate::scene::{Scene, State};
use crate::scene::introduction_scene::{IntroductionScene, IntroductionSceneState};
use crate::scene::main_scene::MainScene;


pub struct SceneWrapper<'a> {
    pub scene: &'a mut dyn Scene,
}
pub enum SceneState {
    Introduction(IntroductionSceneState),
    Main(MainScene),
}

#[no_mangle]
pub extern "C" fn hot_reload_draw_frame(scene_wrapper: &mut Box<Option<SceneState>>) -> State {
    let wrapper = scene_wrapper.take().unwrap();
    match wrapper {
        SceneState::Introduction(scene_state) => {
            let mut scene = IntroductionScene {
                state: scene_state,
            };
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
