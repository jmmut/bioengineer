use clap::Parser;
use macroquad::window::next_frame;
use macroquad::window::Conf;

use bioengineer::common::cli::CliArgs;
use bioengineer::external::backends::{factory, introduction_factory};
use logic::scene::State;
use logic::world::map::chunk::chunks::cache::print_cache_stats;
use logic::{frame, SceneState};
use mq_basics::{now, Seconds};

const DEFAULT_WINDOW_WIDTH: i32 = 1365;
const DEFAULT_WINDOW_HEIGHT: i32 = 768;
const DEFAULT_WINDOW_TITLE: &str = "Bioengineer";

#[macroquad::main(window_conf)]
async fn main() {
    let args = CliArgs::parse();
    let mut scene = introduction_factory(&args).await;
    let mut previous_time = now();
    while frame(&mut scene) == State::ShouldContinue {
        sleep_until_next_frame(&mut previous_time).await
    }
    next_frame().await;

    let mut scene = factory(&args, scene.unwrap().take_textures()).await;
    while frame(&mut scene) == State::ShouldContinue {
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

async fn sleep_until_next_frame(previous_time: &mut Seconds) {
    #[cfg(not(target_family = "wasm"))]
    {
        const MAX_FPS: f64 = 80.0;
        const FRAME_PERIOD: f64 = 1.0 / MAX_FPS;
        let new_time = now();
        // dbg!(new_time);
        // dbg!(*previous_time);
        let frame_duration = new_time - *previous_time;
        if frame_duration < FRAME_PERIOD {
            let sleep_secs = FRAME_PERIOD - frame_duration;
            // info!("sleeping for {}", sleep_secs);

            // this is a blocking sleep on purpose. My current understanding is that macroquad
            // relies on OS or GPU drivers to limit the FPS to ~60 on non-wasm, which doesn't always
            // work. I was experiencing ~8000 FPS and this is the only way I know to limit them.
            // This may not work in web.
            std::thread::sleep(std::time::Duration::from_secs_f64(sleep_secs));
        }
    }
    next_frame().await;
    *previous_time = now();
}