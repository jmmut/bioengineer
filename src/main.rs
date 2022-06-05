mod drawing;
mod game_state;
mod input;
mod map;
mod external {
    pub mod assets_macroquad;
    pub mod drawing_macroquad;
    pub mod input_macroquad;
}

use macroquad::color::Color;
use macroquad::math::{IVec2, IVec3, Vec2, Vec3};
use macroquad::miniquad::date::now;
use macroquad::prelude::next_frame;
use macroquad::texture::Image;
use macroquad::texture::Texture2D;

use external::assets_macroquad::load_tileset;
use external::drawing_macroquad::DrawingMacroquad as DrawingImpl;
use external::input_macroquad::InputMacroquad as InputSource;

use drawing::DrawingTrait;
use game_state::GameState;
use input::InputSourceTrait;
use crate::drawing::{apply_input, draw};

struct Implementations<D: DrawingTrait, I: InputSourceTrait> {
    drawer: D,
    game_state: GameState,
    input: I,
}

#[macroquad::main("Bioengineer")]
async fn main() {
    let Implementations {
        mut drawer,
        mut game_state,
        mut input,
    } = factory().await;

    while frame(&mut game_state, &mut drawer, &mut input) {
        next_frame().await
    }
}

async fn factory() -> Implementations<DrawingImpl, InputSource> {
    let tileset = load_tileset("assets/image/tileset.png");
    let drawer = DrawingImpl::new(tileset.await);
    let game_state = GameState::new();
    let input = InputSource::new();
    Implementations {
        drawer,
        game_state,
        input,
    }
}

/// returns if should continue looping. In other words, if there should be another future frame.
fn frame(
    game_state: &mut GameState,
    drawer: &mut impl DrawingTrait,
    input: &mut impl InputSourceTrait,
) -> bool {
    let input = input.get_input();
    if !input.quit {
        apply_input(drawer, &input);
        draw(drawer, &game_state);
    }
    game_state.advance_frame();
    !input.quit
}
