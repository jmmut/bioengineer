mod common;
mod gui;
mod gui_actions;
mod input;
mod screen;
mod world;

mod external {
    pub mod assets_macroquad;
    pub mod drawing_macroquad;
    pub mod input_macroquad;
}

use macroquad::color::Color;
use macroquad::math::{IVec2, IVec3, Rect, Vec2, Vec3};
use macroquad::miniquad::date::now;
use macroquad::prelude::next_frame;
use macroquad::texture::Image;
use macroquad::texture::Texture2D;
use macroquad::window::Conf;

use external::assets_macroquad::load_tileset;
use external::drawing_macroquad::DrawingMacroquad as DrawingImpl;
use external::input_macroquad::InputMacroquad as InputSource;

use crate::common::profiling::ScopedProfiler;
use crate::gui::Gui;
use crate::screen::Screen;
use crate::world::World;
use screen::drawing::{draw, DrawerTrait};
use world::game_state::GameState;
use input::InputSourceTrait;

const DEFAULT_WINDOW_WIDTH: i32 = 1600;
const DEFAULT_WINDOW_HEIGHT: i32 = 900;
const DEFAULT_WINDOW_TITLE: &'static str = "Bioengineer";

#[macroquad::main(window_conf)]
async fn main() {
    let (mut screen, mut world) = factory().await;

    while frame(&mut screen, &mut world) {
        next_frame().await
    }
}

fn window_conf() -> Conf {
    Conf {
        window_title: DEFAULT_WINDOW_TITLE.to_owned(),
        window_width: DEFAULT_WINDOW_WIDTH,
        window_height: DEFAULT_WINDOW_HEIGHT,
        ..Default::default()
    }
}

async fn factory() -> (Screen<DrawingImpl, InputSource>, World) {
    let tileset = load_tileset("assets/image/tileset.png");
    let mut drawer = DrawingImpl::new(tileset.await);
    let input_source = InputSource::new();
    (Screen::new(drawer, input_source), World::new())
}

/// returns if should continue looping. In other words, if there should be another future frame.
fn frame<D: DrawerTrait, I: InputSourceTrait>(
    screen: &mut Screen<D, I>,
    world: &mut World,
) -> bool {
    let _profiler = ScopedProfiler::new_named(world.game_state.profile, "whole toplevel frame");
    screen.draw(&world);
    let gui_actions = screen.get_gui_actions(&world);
    let should_continue = world.update(gui_actions);
    should_continue
}
