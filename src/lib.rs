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


pub struct SceneWrapper<'a> {
    pub scene: &'a mut dyn Scene,
}
#[no_mangle]
pub extern "C" fn hot_reload_draw_frame(scene_wrapper: &mut SceneWrapper) -> State {
    scene_wrapper.scene.frame()
}
