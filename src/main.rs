mod drawing;
mod input;
mod game_state;

use macroquad::prelude::next_frame;

use drawing::drawing_macroquad::DrawingMacroquad;
use drawing::DrawingTrait;
use input::input_macroquad::InputMacroquad as InputSource;
use input::InputSourceTrait;
use game_state::GameState;

#[macroquad::main("Bioengineer")]
async fn main() {
    let drawer = DrawingMacroquad::new("assets/image/tileset.png");
    let mut game_state = GameState::new();
    while frame(&mut game_state, &drawer) {
        next_frame().await
    }
}

/// returns if should continue looping
fn frame(game_state: &mut GameState, drawer: &impl DrawingTrait) -> bool {
    let input = InputSource::get_input();
    if !input.quit {
        drawer.draw(&game_state);
    }
    game_state.advance_frame();
    !input.quit
}
