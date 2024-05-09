//! This executable allows running the game, change the code, and apply the change to the running
//! process. This feature is called hot reloading. The basic idea is that this file (the entry
//! point) initializes the objects of the game, and then updates them in the game loop, with
//! the twist that each game loop iteration (or frame) is implemented in a dynamic library (.so),
//! so we can recompile the library and reload it with `dlopen()`.
//!
//! As we store the state as variables in the main function, we can reload the functions that
//! transform the state each frame.
//!
//! This has some limitations because everything before the frame function (e.g. initialization)
//! can not be hot-reloaded. If a struct is changed, it will crash the game because the struct
//! that was initialized in `main()` doesn't match the struct that the frame function expects. Also,
//! we do dependency injection for the graphics backend, which means we instantiate the concrete
//! implementation in `main()`, so we can not hot-reload the code for those concrete implementations
//! of the graphics (and input) interfaces. (We can not create the Macroquad backend in the dynamic
//! library because Macroquad has some global variables that don't play nice with hot-reloading.)
//!
//! But this setup is still very useful to quickly iterate things like UI arrangement, game
//! mechanics or small business rules. It's also great for debugging when you need to do several
//! steps in-game to reach the reproduction point. You run the hot-reload-bioengineer in the
//! debugger, prepare the reproduction point, setup breakpoints, analyse the issue with the
//! debugger, change the code, rebuild and reload the lib, hit the breakpoint again, repeat if the
//! bug is still there.
//!
//! In practice, to use this you have to run this binary: `cargo run --bin hot_reload_bioengineer`,
//! and then after changing things in the "logic" crate, you do `cargo build --lib --package logic`.
//! The program will auto-detect that the .so was updated on disk and will reload it. On my machine
//! it takes ~0.5 seconds from starting the build of the library until I see the changes live, for
//! simple changes.
//!
//! Most of these ideas about hot-reloading came from <https://fasterthanli.me/articles/so-you-want-to-live-reload-rust>
//!
//! See <https://jmmut.github.io/2023/03/17/Hot-reloading-Rust-and-Macroquad.html> for the specifics
//! of hot-reloading Macroquad.

use bioengineer::common::cli::{CliArgs, UiBackend};
use bioengineer::external::assets_macroquad::load_tileset;
use clap::Parser;
use juquad::fps::sleep_until_next_frame;
use macroquad::input::is_key_pressed;
use macroquad::logging::info;
use macroquad::window::next_frame;
use macroquad::window::Conf;
use notify::{Event, RecommendedWatcher, RecursiveMode, Watcher};
use std::ffi::{c_char, c_int, c_void, CString};
use std::path::PathBuf;
use std::sync::mpsc::Receiver;

use bioengineer::external::backends::{
    create_introduction_scene, create_main_scene, drawer_factory, TILESET_PATH,
};
use logic::scene::GameLoopState;
use logic::screen::gui::set_skin;
use logic::world::map::chunk::chunks::cache::print_cache_stats;
use logic::SceneState;
use mq_basics::{now, KeyCode};

const DEFAULT_WINDOW_WIDTH: i32 = 1200;
const DEFAULT_WINDOW_HEIGHT: i32 = 675;
const DEFAULT_WINDOW_TITLE: &str = "Hot Reload Bioengineer";

// had to look that one up in `dlfcn.h`
// in C, it's a #define. in Rust, it's a proper constant
pub const RTLD_LAZY: c_int = 0x00001;

type AnyError = Box<dyn std::error::Error>;

pub type DrawFrameFunction = extern "C" fn(scene_wrapper: &mut Box<SceneState>) -> GameLoopState;

#[link(name = "dl")]
extern "C" {
    fn dlopen(path: *const c_char, flags: c_int) -> *const c_void;
    fn dlsym(handle: *const c_void, name: *const c_char) -> *const c_void;
    fn dlclose(handle: *const c_void);
}

#[macroquad::main(window_conf)]
async fn main() -> Result<(), AnyError> {
    let mut args = CliArgs::parse();
    let (mut draw_frame, mut lib_handle) = load()?;
    let (_watcher, rx) = watch()?;

    let mut previous_time = now();
    let textures = {
        let mut scene = create_introduction_scene(&args).await;
        while draw_frame(&mut scene) == GameLoopState::ShouldContinue {
            if should_reload(&rx) {
                info!("reloading lib");
                (draw_frame, lib_handle) = reload(lib_handle)?;
            }
            sleep_until_next_frame(&mut previous_time).await
        }
        next_frame().await;
        scene.take_textures()
    };

    {
        let mut scene = create_main_scene(&args, textures).await;
        while draw_frame(&mut scene) == GameLoopState::ShouldContinue {
            if should_reload(&rx) {
                info!("reloading lib");
                (draw_frame, lib_handle) = reload(lib_handle)?;
            }
            if is_key_pressed(KeyCode::T) {
                scene
                    .as_mut()
                    .set_textures(load_tileset(TILESET_PATH).await);
            }
            if is_key_pressed(KeyCode::Q) {
                swap_ui_backend(scene.as_mut(), &mut args, UiBackend::Macroquad);
            }
            if is_key_pressed(KeyCode::E) {
                swap_ui_backend(scene.as_mut(), &mut args, UiBackend::Egui);
            }
            sleep_until_next_frame(&mut previous_time).await
        }
        if let SceneState::Main(main_scene) = *scene {
            print_cache_stats(main_scene.world.game_state.profile)
        }
    }

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

fn load() -> Result<(DrawFrameFunction, *const c_void), AnyError> {
    let function_name = "hot_reload_draw_frame";
    let lib_name = "target/debug/liblogic.so";

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

fn should_reload(rx: &Receiver<()>) -> bool {
    rx.try_recv().is_ok()

    // use macroquad::prelude::draw_text;
    // use macroquad::color::BLACK;
    // use macroquad::input::{is_key_down, is_key_released, KeyCode};
    // if is_key_down(KeyCode::F5) {
    //     draw_text("About to reload when you release", 20.0, 20.0, 30.0, BLACK);
    // }
    // is_key_released(KeyCode::F5)
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

fn watch() -> Result<(RecommendedWatcher, Receiver<()>), AnyError> {
    let base = PathBuf::from(".").canonicalize().unwrap();
    let libname = "liblogic.so";
    let relative_path = PathBuf::from("target").join("debug").join(libname);
    // let absolute_path = base.join(&relative_path);

    // here's our watcher to communicate between the watcher thread
    // (using `tx`, the "transmitter") and the main thread (using
    // `rx`, the "receiver").
    let (tx, rx) = std::sync::mpsc::channel::<()>();

    let mut watcher = notify::recommended_watcher(move |res: Result<Event, notify::Error>| {
        match res {
            Ok(event) => {
                if let notify::EventKind::Create(_) = event.kind {
                    if event.paths.iter().any(|x| x.ends_with(&relative_path)) {
                        // signal that we need to reload
                        tx.send(()).unwrap();
                    }
                }
            }
            Err(e) => println!("watch error: {:?}", e),
        }
    })?;

    watcher.watch(&base, RecursiveMode::Recursive).unwrap();
    Ok((watcher, rx))
}

fn swap_ui_backend(scene: &mut SceneState, args: &mut CliArgs, new_ui_backend: UiBackend) {
    if let SceneState::Main(main_scene) = scene {
        args.ui = new_ui_backend; // keep the latest chosen ui backend. Used when re-creating by calling to the full factory (KC::F)
        let mut tmp_drawer = drawer_factory(new_ui_backend, Vec::new());
        std::mem::swap(&mut tmp_drawer, &mut main_scene.screen.drawer);
        main_scene
            .screen
            .drawer
            .set_textures(tmp_drawer.take_textures());
        set_skin(main_scene.screen.drawer.as_mut()); // normally Gui::new() will do this but we're not recreating that here
    }
}
