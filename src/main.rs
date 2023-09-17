use clap::Parser;
use macroquad::window::next_frame;
use macroquad::window::Conf;

use bioengineer::common::cli::CliArgs;
use bioengineer::external::backends::{factory, introduction_factory};
use bioengineer::frame;
use bioengineer::scene::{Scene, State};
use bioengineer::world::map::chunk::chunks::cache::print_cache_stats;

const DEFAULT_WINDOW_WIDTH: i32 = 1365;
const DEFAULT_WINDOW_HEIGHT: i32 = 768;
const DEFAULT_WINDOW_TITLE: &str = "Bioengineer";

#[macroquad::main(window_conf)]
async fn main() {
    let args = CliArgs::parse();
    let mut scene = introduction_factory(&args).await;
    while scene.frame() == State::ShouldContinue {
        next_frame().await
    }

    let (mut screen, mut world) = factory(&args).await;
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
