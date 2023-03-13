mod common{
    pub mod profiling;
    pub mod trunc;
}
pub mod screen;
pub mod world;

pub mod external {
    pub mod assets_macroquad;
    pub mod drawer_macroquad;
    pub mod input_macroquad;
}

use macroquad::color::Color;
use macroquad::math::{IVec2, IVec3, Rect, Vec2, Vec3};
use macroquad::miniquad::date::now;
use macroquad::texture::{Image, Texture2D};

use common::profiling::ScopedProfiler;
use screen::Screen;
use world::World;


/// returns if should continue looping. In other words, if there should be another future frame.
pub fn frame(
    screen: &mut Screen,
    world: &mut World,
) -> bool {
    let _profiler = ScopedProfiler::new_named(world.game_state.profile, "whole toplevel frame");
    let gui_actions = screen.get_gui_actions(world);
    let should_continue = world.update(gui_actions);
    screen.draw(world);
    should_continue
}


#[no_mangle]
pub extern "C" fn hot_reload_draw_frame(
    screen: &mut Screen,
    world: &mut World,
) -> bool  {
    frame(screen, world)
}
