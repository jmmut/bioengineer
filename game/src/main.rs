use clap::Parser;
use macroquad::window::next_frame;
use macroquad::window::Conf;

use bioengineer::common::cli::CliArgs;
use bioengineer::external::backends::{factory, introduction_factory};
use juquad::fps::sleep_until_next_frame;
use logic::scene::GameLoopState;
use logic::world::map::chunk::chunks::cache::print_cache_stats;
use logic::{frame, SceneState};
use mq_basics::now;

const DEFAULT_WINDOW_WIDTH: i32 = 1365;
const DEFAULT_WINDOW_HEIGHT: i32 = 768;
const DEFAULT_WINDOW_TITLE: &str = "Bioengineer";

#[macroquad::main(window_conf)]
async fn main() {
    let args = CliArgs::parse();
    let mut scene = introduction_factory(&args).await;
    let mut previous_time = now();
    while frame(&mut scene) == GameLoopState::ShouldContinue {
        sleep_until_next_frame(&mut previous_time).await
    }
    next_frame().await;

    let mut scene = factory(&args, scene.unwrap().take_textures()).await;
    while frame(&mut scene) == GameLoopState::ShouldContinue {
        sleep_until_next_frame(&mut previous_time).await
    }
    if let SceneState::Main(main_scene) = scene.as_ref().as_ref().unwrap() {
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
