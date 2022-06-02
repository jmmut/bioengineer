mod drawing;
mod game_state;
mod input;

use macroquad::prelude::next_frame;

use drawing::drawing_macroquad::DrawingMacroquad;
use drawing::DrawingTrait;
use game_state::GameState;
use input::input_macroquad::InputMacroquad as InputSource;
use input::InputSourceTrait;

struct Implementations<D: DrawingTrait> {
    drawer: D,
    game_state: GameState,
}

#[macroquad::main("Bioengineer")]
async fn main() {
    let Implementations {
        drawer,
        mut game_state,
    } = factory();

    while frame(&mut game_state, &drawer) {
        next_frame().await
    }
}

fn factory() -> Implementations<DrawingMacroquad> {
    let drawer = DrawingMacroquad::new("assets/image/tileset.png");
    let game_state = GameState::new();
    Implementations { drawer, game_state }
}

/// returns if should continue looping. In other words, if there should be another future frame.
fn frame(game_state: &mut GameState, drawer: &impl DrawingTrait) -> bool {
    let input = InputSource::get_input();
    if !input.quit {
        drawer.draw(&game_state);
    }
    game_state.advance_frame();
    !input.quit
}
