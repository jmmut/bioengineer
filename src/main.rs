mod drawing;
mod game_state;
mod gui;
mod input;
mod map;
mod profiling;

mod external {
    pub mod assets_macroquad;
    pub mod drawing_macroquad;
    pub mod input_macroquad;
}

use macroquad::color::Color;
use macroquad::math::{IVec2, IVec3, Rect, Vec2, Vec3};
use macroquad::miniquad::date::now;
use macroquad::prelude::next_frame;
use macroquad::texture::Image;
use macroquad::texture::Texture2D;
use macroquad::window::Conf;

use external::assets_macroquad::load_tileset;
use external::drawing_macroquad::DrawingMacroquad as DrawingImpl;
use external::input_macroquad::InputMacroquad as InputSource;

use crate::gui::Gui;
use crate::profiling::ScopedProfiler;
use drawing::{draw, DrawingTrait};
use game_state::GameState;
use input::InputSourceTrait;

const DEFAULT_WINDOW_WIDTH: i32 = 1600;
const DEFAULT_WINDOW_HEIGHT: i32 = 900;
const DEFAULT_WINDOW_TITLE: &'static str = "Bioengineer";

#[macroquad::main(window_conf)]
async fn main() {
    let mut implementations = factory().await;

    while frame(&mut implementations) {
        next_frame().await
    }
}

fn window_conf() -> Conf {
    Conf {
        window_title: DEFAULT_WINDOW_TITLE.to_owned(),
        window_width: DEFAULT_WINDOW_WIDTH,
        window_height: DEFAULT_WINDOW_HEIGHT,
        ..Default::default()
    }
}

struct Implementations<D: DrawingTrait, I: InputSourceTrait> {
    drawer: D,
    game_state: GameState,
    input: I,
    gui: Gui,
}

async fn factory() -> Implementations<DrawingImpl, InputSource> {
    let tileset = load_tileset("assets/image/tileset.png");
    let mut drawer = DrawingImpl::new(tileset.await);
    let game_state = GameState::new();
    let input = InputSource::new();
    let gui = gui::Gui::new(&mut drawer);
    Implementations {
        drawer,
        game_state,
        input,
        gui,
    }
}

/// returns if should continue looping. In other words, if there should be another future frame.
fn frame<D: DrawingTrait, I: InputSourceTrait>(
    implementations: &mut Implementations<D, I>,
) -> bool {
    let game_state = &mut implementations.game_state;
    let _profiler = ScopedProfiler::new_named(game_state.profile, "whole toplevel frame");
    let drawer = &mut implementations.drawer;
    let input_source = &mut implementations.input;
    let gui = &mut implementations.gui;

    let input = input_source.get_input();
    let should_continue = !input.quit;
    if should_continue {
        draw(drawer, &game_state);
        let gui_actions = gui.receive_actions(input, drawer, &game_state);
        game_state.update_with_gui_actions(&gui_actions);
        game_state
            .get_drawing_mut()
            .apply_input(&gui_actions, drawer.screen_width());
    }
    game_state.advance_frame();
    should_continue
}
