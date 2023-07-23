//! Most of these ideas came from https://fasterthanli.me/articles/so-you-want-to-live-reload-rust

use bioengineer::common::cli::CliArgs;
use clap::Parser;
use macroquad::window::next_frame;
use macroquad::window::Conf;
use notify::{Event, RecommendedWatcher, RecursiveMode, Watcher};
use std::ffi::{c_char, c_int, c_void, CString};
use std::path::PathBuf;
use std::sync::mpsc::Receiver;

use bioengineer::external::backends::factory;
use bioengineer::screen::Screen;
use bioengineer::world::map::chunk::chunks::cache::print_cache_stats;
use bioengineer::world::World;

const DEFAULT_WINDOW_WIDTH: i32 = 1365;
const DEFAULT_WINDOW_HEIGHT: i32 = 768;
const DEFAULT_WINDOW_TITLE: &str = "Hot Reload Bioengineer";

// had to look that one up in `dlfcn.h`
// in C, it's a #define. in Rust, it's a proper constant
pub const RTLD_LAZY: c_int = 0x00001;

type AnyError = Box<dyn std::error::Error>;

pub type DrawFrameFunction = extern "C" fn(screen: &mut Screen, world: &mut World) -> bool;

#[link(name = "dl")]
extern "C" {
    fn dlopen(path: *const c_char, flags: c_int) -> *const c_void;
    fn dlsym(handle: *const c_void, name: *const c_char) -> *const c_void;
    fn dlclose(handle: *const c_void);
}

#[macroquad::main(window_conf)]
async fn main() -> Result<(), AnyError> {
    let args = CliArgs::parse();
    let (mut screen, mut world) = factory(&args).await; // TODO: reload screen too (textures)
    let (_watcher, rx) = watch()?;
    let (mut draw_frame, mut lib_handle) = load()?;
    while draw_frame(&mut screen, &mut world) {
        if should_reload(&rx) {
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

fn load() -> Result<(DrawFrameFunction, *const c_void), AnyError> {
    let function_name = "hot_reload_draw_frame";
    let lib_name = "target/debug/libbioengineer.so";

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
    let libname = "libbioengineer.so";
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
