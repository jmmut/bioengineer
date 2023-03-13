use std::ffi::{c_char, c_int, c_void, CString};
use clap::Parser;
use git_version::git_version;
use macroquad::color::BLACK;
use macroquad::input::{is_key_down, is_key_released, KeyCode};
use macroquad::prelude::draw_text;
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
pub const RTLD_LAZY: c_int = 0x00001;

type AnyError = Box<dyn std::error::Error>;

pub type DrawFrameFunction = extern "C" fn(
    screen: &mut Screen,
    world: &mut World,
) -> bool;

#[link(name = "dl")]
extern "C" {
    fn dlopen(path: *const c_char, flags: c_int) -> *const c_void;
    fn dlsym(handle: *const c_void, name: *const c_char) -> *const c_void;
    fn dlclose(handle: *const c_void);
}


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
async fn main() -> Result<(), AnyError> {
    let (mut screen, mut world) = factory().await;

    let (mut draw_frame, mut lib_handle) = load()?;
    while frame(&mut screen, &mut world) {

        if should_reload() {
            (draw_frame, lib_handle) = reload(lib_handle)?;
        }
        next_frame().await
    }
    print_cache_stats(world.game_state.profile);
    unload(lib_handle);
    Ok(())
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

fn load() -> Result<(DrawFrameFunction, *const c_void), AnyError> {
    let function_name = "frame";
    let lib_name= "target/debug/libbioengineer.so";

    let lib_name = CString::new(lib_name).unwrap();
    let lib = unsafe { dlopen(lib_name.as_ptr(), RTLD_LAZY) };
    if lib.is_null() {
        return Err(format!(
            "could not open library at {} from {}",
            lib_name.to_str().unwrap(),
            std::env::current_dir()?.to_str().unwrap()
        )
            .into());
    }

    let function_name = CString::new(function_name).unwrap();
    let function = unsafe { dlsym(lib, function_name.as_ptr()) };
    if function.is_null() {
        return Err(format!(
            "could not load function {}",
            function_name.to_str().unwrap()
        )
            .into());
    }
    use std::mem::transmute;
    let transmuted_function: DrawFrameFunction = unsafe { transmute(function) };
    Ok((transmuted_function, lib))
}

fn should_reload() -> bool {
    if is_key_down(KeyCode::R) {
        draw_text("About to reload when you release", 20.0, 20.0, 30.0, BLACK);
    }
    is_key_released(KeyCode::R)
}

fn reload(lib: *const c_void) -> Result<(DrawFrameFunction, *const c_void), AnyError> {
    unload(lib);
    load()
}

fn unload(lib: *const c_void) {
    if !lib.is_null() {
        unsafe {
            dlclose(lib);
        }
    }
}
