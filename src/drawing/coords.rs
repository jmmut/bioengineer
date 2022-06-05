pub mod cast;
pub mod cell_tile;
pub mod tile_pixel;
pub mod truncate;

use crate::drawing::assets::{PIXELS_PER_TILE_HEIGHT, PIXELS_PER_TILE_WIDTH};
use crate::drawing::coords::cell_tile::{
    cell_to_tile, cell_to_tile_unwrapped, subcell_to_subtile, subtile_to_subcell,
    subtile_to_subcell_offset, tile_to_cell,
};
use crate::drawing::coords::tile_pixel::{
    pixel_to_subtile, pixel_to_subtile_offset, pixel_to_tile, subtile_to_pixel, tile_to_pixel,
};
use crate::drawing::{assets, Drawing, PixelPosition, SubCellIndex, SubTilePosition, TilePosition};
use crate::map::CellIndex;
use crate::DrawingTrait;

pub fn cell_to_pixel(cell_index: CellIndex, drawing: &Drawing, screen_width: f32) -> PixelPosition {
    let tile = cell_to_tile(cell_index, &drawing.min_cell, &drawing.max_cell);
    tile_to_pixel(tile, drawing, screen_width)
}

pub fn pixel_to_cell(
    pixel_position: PixelPosition,
    drawing: &Drawing,
    screen_width: f32,
) -> CellIndex {
    let tile = pixel_to_tile(pixel_position, drawing, screen_width);
    let cell_index = tile_to_cell(tile, &drawing.min_cell, &drawing.max_cell);
    cell_index
}

pub fn pixel_to_subcell(
    pixel_position: PixelPosition,
    drawing: &Drawing,
    screen_width: f32,
) -> SubCellIndex {
    let subtile = pixel_to_subtile(pixel_position, drawing, screen_width);
    let cell_index = subtile_to_subcell(subtile, &drawing.min_cell, &drawing.max_cell);
    cell_index
}

pub fn pixel_to_subcell_offset(pixel_diff: PixelPosition) -> SubCellIndex {
    let subtile = pixel_to_subtile_offset(pixel_diff);
    subtile_to_subcell_offset(subtile)
}

pub fn pixel_to_subcell_center(
    pixel: PixelPosition,
    drawing: &Drawing,
    screen_width: f32,
) -> SubCellIndex {
    let mut subtile = pixel_to_subtile(pixel, drawing, screen_width);

    // move the hitbox to the center of the tile
    let subtile_center = subtile - tile_offset();

    let subcell = subtile_to_subcell(subtile_center, &drawing.min_cell, &drawing.max_cell);
    subcell
}

pub fn subcell_center_to_pixel(
    subcell: SubCellIndex,
    drawing: &Drawing,
    screen_width: f32,
) -> PixelPosition {
    let subtile = subcell_to_subtile(subcell, &drawing.min_cell, &drawing.max_cell);

    // move the hitbox to the center of the tile
    let subtile = subtile + tile_offset();

    subtile_to_pixel(subtile, drawing, screen_width)
}

fn tile_offset() -> SubTilePosition {
    SubTilePosition::new(1.0, 1.0)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::drawing::TilePosition;
    use crate::IVec3;

    fn cell_to_pixel_to_cell(initial_cell: CellIndex) {
        let mut drawing = Drawing::new();
        drawing.max_cell.y = initial_cell.y;
        let screen_width = 800.0;
        let pixel = cell_to_pixel(initial_cell, &drawing, screen_width);
        let final_cell = pixel_to_cell(pixel, &drawing, screen_width);
        assert_eq!(final_cell, initial_cell);
    }

    #[test]
    fn test_cell_to_pixel_to_cell() {
        cell_to_pixel_to_cell(CellIndex::new(0, 0, 0));
        cell_to_pixel_to_cell(CellIndex::new(1, 0, 0));
        cell_to_pixel_to_cell(CellIndex::new(0, 1, 0));
        cell_to_pixel_to_cell(CellIndex::new(0, 0, 1));
    }

    fn pixel_to_subcell_to_pixel(initial_pixel: PixelPosition) {
        let mut drawing = Drawing::new();
        let screen_width = 800.0;
        let subcell = pixel_to_subcell_center(initial_pixel, &drawing, screen_width);
        let final_pixel = subcell_center_to_pixel(subcell, &drawing, screen_width);
        assert_eq!(final_pixel, initial_pixel);
    }

    #[test]
    fn test_pixel_to_subcell_to_pixel() {
        pixel_to_subcell_to_pixel(PixelPosition::new(0.0, 0.0));
        pixel_to_subcell_to_pixel(PixelPosition::new(1.0, 0.0));
        pixel_to_subcell_to_pixel(PixelPosition::new(0.0, 1.0));
        pixel_to_subcell_to_pixel(PixelPosition::new(100.0, 0.0));
        pixel_to_subcell_to_pixel(PixelPosition::new(0.0, 100.0));
    }
}
