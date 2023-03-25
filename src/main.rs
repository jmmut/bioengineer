use clap::Parser;
use futures::executor::block_on;
use git_version::git_version;
use macroquad::miniquad;
use macroquad::window::next_frame;
use macroquad::window::Conf;

use bioengineer::external::assets_macroquad::load_tileset;
use bioengineer::external::drawer_egui_macroquad::DrawerEguiMacroquad as DrawerImpl;
use bioengineer::external::drawer_egui_miniquad::Stage;
use bioengineer::external::input_macroquad::InputMacroquad as InputSource;
use bioengineer::frame;
use bioengineer::screen::drawer_trait::DrawerTrait;
use bioengineer::screen::Screen;
use bioengineer::world::map::chunk::chunks::cache::print_cache_stats;
use bioengineer::world::World;

const DEFAULT_WINDOW_WIDTH: i32 = 1365;
const DEFAULT_WINDOW_HEIGHT: i32 = 768;
const DEFAULT_WINDOW_TITLE: &str = "Bioengineer";

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

// #[macroquad::main("egui with macroquad")]
// fn main() {
//
//     let (screen, world) = block_on(factory());
//     miniquad::start(conf, |mut ctx| Box::new(Stage::new(&mut ctx, screen, world)));
//     // print_cache_stats(world.game_state.profile); // TODO: is this at the end of the program or end of init?
// }

// async fn main_loop_async() {
//
//     while frame(&mut screen, &mut world) {
//         next_frame().await
//     }
//
// }

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
        // high_dpi: true,
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

// #[macroquad::main("egui with macroquad")]
async fn main2() {
    loop {
        // clear_background(WHITE);

        // Process keys, mouse etc.
        //
        // egui_macroquad::ui(|egui_ctx| {
        //     egui::Window::new("egui ❤ macroquad")
        //         .show(egui_ctx, |ui| {
        //             ui.label("Test");
        //         });
        // });

        // Draw things before egui

        // egui_macroquad::draw();

        // Draw things after egui

        // next_frame().await;
    }
}
