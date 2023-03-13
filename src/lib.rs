mod common;
mod screen;
mod world;

mod external {
    pub mod assets_macroquad;
    pub mod drawer_macroquad;
    pub mod input_macroquad;
}

use common::profiling::ScopedProfiler;



use clap::Parser;
use macroquad::color::Color;
use macroquad::math::{IVec2, IVec3, Rect, Vec2, Vec3};
use macroquad::miniquad::date::now;
use macroquad::texture::{Image, Texture2D};
use macroquad::window::next_frame;
use macroquad::window::Conf;

use external::assets_macroquad::load_tileset;
use external::drawer_macroquad::DrawerMacroquad as DrawerImpl;
use external::input_macroquad::InputMacroquad as InputSource;

use screen::drawer_trait::DrawerTrait;
// use screen::gui::Gui;
use crate::world::map::chunk::chunks::cache::print_cache_stats;
use screen::input::InputSourceTrait;
use screen::Screen;
use world::game_state::GameState;
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
