mod assets;
pub mod drawing_macroquad;

use super::game_state::GameState;

pub trait DrawingTrait {
    fn new(tileset_path: &str) -> Self;
    fn draw(&self, game_state: &GameState);
}
