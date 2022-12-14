mod common;
mod screen;
mod world;

mod external {
    pub mod assets_macroquad;
    pub mod drawer_macroquad;
    pub mod input_macroquad;
}

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

use common::profiling::ScopedProfiler;
use screen::drawer_trait::DrawerTrait;
// use screen::gui::Gui;
use crate::world::map::chunk::chunks::cache::print_cache_stats;
use screen::input::InputSourceTrait;
use screen::Screen;
use world::game_state::GameState;
use world::World;

const DEFAULT_WINDOW_WIDTH: i32 = 1600;
const DEFAULT_WINDOW_HEIGHT: i32 = 900;
const DEFAULT_WINDOW_TITLE: &str = "Bioengineer";

use git_version::git_version;
const GIT_VERSION: &str = git_version!(args = ["--tags"]);

#[derive(Parser, Debug)]
#[clap(version = GIT_VERSION)]
struct CliArgs {
    #[clap(long, help = "Measure and print profiling information.")]
    profile: bool,

    #[clap(
        long,
        help = "Enable fluid simulation. Game will have worse performance."
    )]
    fluids: bool,
}

#[macroquad::main(window_conf)]
async fn main() {
    let (mut screen, mut world) = factory().await;

    while frame(&mut screen, &mut world) {
        next_frame().await
    }
    print_cache_stats(world.game_state.profile);
}

fn window_conf() -> Conf {
    Conf {
        window_title: DEFAULT_WINDOW_TITLE.to_owned(),
        window_width: DEFAULT_WINDOW_WIDTH,
        window_height: DEFAULT_WINDOW_HEIGHT,
        ..Default::default()
    }
}

async fn factory() -> (Screen<DrawerImpl, InputSource>, World) {
    let args = CliArgs::parse();
    println!("Running Bioengineer version {}", GIT_VERSION);
    let tileset = load_tileset("assets/image/tileset.png");
    let drawer = DrawerImpl::new(tileset.await);
    let input_source = InputSource::new();
    let world = World::new_with_options(args.profile);
    (Screen::new(drawer, input_source), world)
}

/// returns if should continue looping. In other words, if there should be another future frame.
fn frame<D: DrawerTrait, I: InputSourceTrait>(
    screen: &mut Screen<D, I>,
    world: &mut World,
) -> bool {
    let _profiler = ScopedProfiler::new_named(world.game_state.profile, "whole toplevel frame");
    let gui_actions = screen.get_gui_actions(world);
    let should_continue = world.update(gui_actions);
    screen.draw(world);
    should_continue
}
