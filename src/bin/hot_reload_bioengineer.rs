use std::ffi::{c_char, c_int, c_void};
use clap::Parser;
use git_version::git_version;
use macroquad::window::Conf;
use macroquad::window::next_frame;

use bioengineer::external::drawer_macroquad::DrawerMacroquad as DrawerImpl;
use bioengineer::external::input_macroquad::InputMacroquad as InputSource;
use bioengineer::external::assets_macroquad::load_tileset;
use bioengineer::frame;
use bioengineer::screen::drawer_trait::DrawerTrait;
use bioengineer::screen::Screen;
use bioengineer::world::map::chunk::chunks::cache::print_cache_stats;
use bioengineer::world::World;

const DEFAULT_WINDOW_WIDTH: i32 = 1365;
const DEFAULT_WINDOW_HEIGHT: i32 = 768;
const DEFAULT_WINDOW_TITLE: &str = "Hot Reload Bioengineer";

const GIT_VERSION: &str = git_version!(args = ["--tags"]);

// had to look that one up in `dlfcn.h`
// in C, it's a #define. in Rust, it's a proper constant
// pub const RTLD_LAZY: c_int = 0x00001;
//
//
// #[link(name = "dl")]
// extern "C" {
//     fn dlopen(path: *const c_char, flags: c_int) -> *const c_void;
//     fn dlsym(handle: *const c_void, name: *const c_char) -> *const c_void;
//     fn dlclose(handle: *const c_void);
// }

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

async fn factory() -> (Screen, World) {
    let args = CliArgs::parse();
    println!("Running Bioengineer version {}", GIT_VERSION);
    let tileset = load_tileset("assets/image/tileset.png");
    let drawer = Box::new(DrawerImpl::new(tileset.await));
    let input_source = Box::new(InputSource::new());
    let world = World::new_with_options(args.profile);
    (Screen::new(drawer, input_source), world)
}
