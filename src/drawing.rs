mod assets;
pub mod drawing_macroquad;

use crate::game_state::GameState;
use crate::map::{Cell, CellIndex, TileType};
use assets::{PIXELS_PER_TILE_HEIGHT, PIXELS_PER_TILE_WIDTH};

pub struct Drawing {
    min_cell: CellIndex,
    max_cell: CellIndex,
}

impl Drawing {
    pub fn new() -> Self {
        Drawing {
            min_cell: CellIndex::new(0, 0, 0),
            max_cell: CellIndex::new(15, 8, 15),
        }
    }
}

pub trait DrawingTrait {
    fn new(tileset_path: &str) -> Self;
    fn draw(&self, game_state: &GameState);
    fn draw_texture(&self, tile: TileType, x: f32, y: f32);
    fn drawing(&self) -> &Drawing;
    fn screen_width(&self) -> f32;
    fn screen_height(&self) -> f32;
    fn draw_map(&self, game_state: &GameState) {
        for i_y in self.drawing().min_cell.y..=self.drawing().max_cell.y {
            for i_z in self.drawing().min_cell.z..=self.drawing().max_cell.z {
                for i_x in self.drawing().min_cell.x..=self.drawing().max_cell.x {
                    let cell_index = CellIndex::new(i_x, i_y, i_z);
                    let (x, y) = self.get_draw_position(i_x, i_y, i_z);
                    self.draw_texture(game_state.map.get_cell(cell_index).tile_type, x, y);
                }
            }
        }
    }

    fn get_draw_position(&self, i_x: i32, i_y: i32, i_z: i32) -> (f32, f32) {
        let (mut x, mut y) = get_tile_position(
            &self.drawing().min_cell,
            &self.drawing().max_cell,
            i_x,
            i_y,
            i_z,
        );
        let pixels_half_tile_x = PIXELS_PER_TILE_WIDTH as f32 * 0.5;
        let pixels_half_tile_y = PIXELS_PER_TILE_HEIGHT as f32 * 0.5;
        let pixels_height_isometric = pixels_half_tile_y * 0.5;
        x = f32::trunc(x * pixels_half_tile_x + self.screen_width() / 2.0 - pixels_half_tile_x);
        y = f32::trunc(y * pixels_height_isometric);
        (x, y)
    }
}

fn get_tile_position(
    min_cell: &CellIndex,
    max_cell: &CellIndex,
    i_x: i32,
    i_y: i32,
    i_z: i32,
) -> (f32, f32) {
    (
        ((i_x - min_cell.x) - (i_z - min_cell.z)) as f32,
        ((i_x - min_cell.x) + (i_z - min_cell.z) + 2 * (max_cell.y - i_y)) as f32,
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use macroquad::math::IVec3;

    #[test]
    fn position_tile_basic() {
        let min_cell = CellIndex::new(0, 0, 0);
        let max_cell = CellIndex::new(10, 10, 10);
        assert_eq!(
            get_tile_position(&min_cell, &max_cell, 0, max_cell.y, 0),
            (0.0, 0.0)
        );
        assert_eq!(
            get_tile_position(&min_cell, &max_cell, 1, max_cell.y, 1),
            (0.0, 2.0)
        );
    }

    #[test]
    fn position_tile_min_cell() {
        let min_cell = CellIndex::new(0, 0, 0);
        let max_cell = CellIndex::new(10, 10, 10);
        let (x, y) = get_tile_position(&min_cell, &max_cell, min_cell.x, max_cell.y, min_cell.z);
        assert_eq!(x, 0.0);
        assert_eq!(y, 0.0);
    }
    #[test]
    fn position_tile_negative() {
        let min_cell = CellIndex::new(-5, -25, -55);
        let max_cell = CellIndex::new(5, -15, -45);
        assert_eq!(
            get_tile_position(&min_cell, &max_cell, min_cell.x, max_cell.y, min_cell.z),
            (0.0, 0.0)
        );
        assert_eq!(
            get_tile_position(
                &min_cell,
                &max_cell,
                min_cell.x + 1,
                max_cell.y,
                min_cell.z + 1
            ),
            (0.0, 2.0)
        );
    }
    #[test]
    fn position_tile_height() {
        let min_cell = CellIndex::new(-5, -25, -55);
        let max_cell = CellIndex::new(5, -15, -45);
        assert_eq!(
            get_tile_position(
                &min_cell,
                &max_cell,
                min_cell.x + 1,
                max_cell.y,
                min_cell.z + 1
            ),
            get_tile_position(&min_cell, &max_cell, min_cell.x, max_cell.y - 1, min_cell.z)
        );
    }
}
