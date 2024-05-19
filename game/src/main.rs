use clap::Parser;
use macroquad::window::next_frame;
use macroquad::window::Conf;

use bioengineer::common::cli::CliArgs;
use bioengineer::external::backends::{create_introduction_scene, create_main_scene};
use juquad::fps::sleep_until_next_frame;
use logic::scene::GameLoopState;
use logic::world::map::chunk::chunks::cache::print_cache_stats;
use logic::{frame, SceneState};
use mq_basics::now;

const DEFAULT_WINDOW_WIDTH: i32 = 1200;
const DEFAULT_WINDOW_HEIGHT: i32 = 675;
const DEFAULT_WINDOW_TITLE: &str = "Bioengineer";

#[macroquad::main(window_conf)]
async fn main() {
    let args = CliArgs::parse();
    let mut scene = create_introduction_scene(&args).await;
    let mut previous_time = now();
    while frame(&mut scene).should_continue() {
        sleep_until_next_frame(&mut previous_time).await
    }
    next_frame().await;

    let mut scene = create_main_scene(&args, scene.take_textures()).await;
    while frame(&mut scene).should_continue() {
        sleep_until_next_frame(&mut previous_time).await
    }
    if let SceneState::Main(main_scene) = scene.as_ref() {
        print_cache_stats(main_scene.world.game_state.profile);
    }
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
