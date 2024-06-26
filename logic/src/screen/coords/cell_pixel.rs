use crate::screen::coords::cell_tile::{
    cell_to_tile, subcell_to_subtile, subtile_to_subcell, subtile_to_subcell_offset, tile_to_cell,
};
use crate::screen::coords::tile_pixel::{
    pixel_to_subtile, pixel_to_subtile_offset, pixel_to_tile, subtile_to_pixel, tile_to_pixel,
};
use crate::screen::coords::truncate::truncate_cell_offset;
use crate::screen::draw_map::hitbox_offset;
use crate::screen::drawing_state::{DrawingState, SubCellIndex, SubTilePosition};
use crate::world::map::CellIndex;
use juquad::PixelPosition;

pub fn clicked_cell(click: PixelPosition, screen_width: f32, drawing: &DrawingState) -> CellIndex {
    let moved_selected = click + hitbox_offset();
    let subcell = pixel_to_subcell_center(moved_selected, drawing, screen_width);
    let (cell, _) = truncate_cell_offset(subcell);
    cell
}

pub fn cell_to_pixel(
    cell_index: CellIndex,
    drawing: &DrawingState,
    screen_width: f32,
) -> PixelPosition {
    let tile = cell_to_tile(cell_index, &drawing.min_cell, &drawing.max_cell);
    tile_to_pixel(tile, drawing, screen_width)
}

#[allow(dead_code)]
pub fn pixel_to_cell(
    pixel_position: PixelPosition,
    drawing: &DrawingState,
    screen_width: f32,
) -> CellIndex {
    let tile = pixel_to_tile(pixel_position, drawing, screen_width);
    let cell_index = tile_to_cell(tile, &drawing.min_cell, &drawing.max_cell);
    cell_index
}

#[allow(dead_code)]
pub fn pixel_to_subcell(
    pixel_position: PixelPosition,
    drawing: &DrawingState,
    screen_width: f32,
) -> SubCellIndex {
    let subtile = pixel_to_subtile(pixel_position, drawing, screen_width);
    let cell_index = subtile_to_subcell(subtile, &drawing.min_cell, &drawing.max_cell);
    cell_index
}

pub fn pixel_to_subcell_offset(pixel_diff: PixelPosition, zoom: f32) -> SubCellIndex {
    let subtile = pixel_to_subtile_offset(pixel_diff, zoom);
    subtile_to_subcell_offset(subtile)
}

pub fn pixel_to_subcell_center(
    pixel: PixelPosition,
    drawing: &DrawingState,
    screen_width: f32,
) -> SubCellIndex {
    let subtile = pixel_to_subtile(pixel, drawing, screen_width);

    // move the hitbox to the center of the tile
    let subtile_center = subtile - tile_offset();

    let subcell = subtile_to_subcell(subtile_center, &drawing.min_cell, &drawing.max_cell);
    subcell
}

pub fn subcell_center_to_pixel(
    subcell: SubCellIndex,
    drawing: &DrawingState,
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
    use crate::screen::assets::{PIXELS_PER_TILE_HEIGHT, PIXELS_PER_TILE_WIDTH};
    use crate::screen::coords::cell_pixel::{
        cell_to_pixel, pixel_to_cell, pixel_to_subcell_center, subcell_center_to_pixel,
    };
    use crate::screen::drawing_state::DrawingState;
    use crate::world::map::CellIndex;

    #[test]
    fn test_pixel_to_cell_offset_basic() {
        let pixel_diff = PixelPosition::new(0.0, 0.0);
        let subcell_diff = pixel_to_subcell_offset(pixel_diff, 1.0);
        assert_eq!(subcell_diff, SubCellIndex::new(0.0, 0.0, 0.0));
    }

    #[test]
    fn test_pixel_to_cell_offset_x() {
        let pixel_diff = PixelPosition::new(PIXELS_PER_TILE_WIDTH as f32, 0.0);
        let subcell_diff = pixel_to_subcell_offset(pixel_diff, 1.0);
        assert_eq!(subcell_diff, SubCellIndex::new(1.0, 0.0, -1.0));

        let pixel_diff = PixelPosition::new(PIXELS_PER_TILE_WIDTH as f32 * 0.5, 0.0);
        let subcell_diff = pixel_to_subcell_offset(pixel_diff, 1.0);
        assert_eq!(subcell_diff, SubCellIndex::new(0.5, 0.0, -0.5));
    }

    #[test]
    fn test_pixel_to_cell_offset_y() {
        let pixel_diff = PixelPosition::new(0.0, PIXELS_PER_TILE_HEIGHT as f32);
        let subcell_diff = pixel_to_subcell_offset(pixel_diff, 1.0);
        assert_eq!(subcell_diff, SubCellIndex::new(2.0, 0.0, 2.0));

        let pixel_diff = PixelPosition::new(0.0, PIXELS_PER_TILE_HEIGHT as f32 * 0.5);
        let subcell_diff = pixel_to_subcell_offset(pixel_diff, 1.0);
        assert_eq!(subcell_diff, SubCellIndex::new(1.0, 0.0, 1.0));

        let pixel_diff = PixelPosition::new(0.0, PIXELS_PER_TILE_HEIGHT as f32 * 0.25);
        let subcell_diff = pixel_to_subcell_offset(pixel_diff, 1.0);
        assert_eq!(subcell_diff, SubCellIndex::new(0.5, 0.0, 0.5));
    }

    fn cell_to_pixel_to_cell(initial_cell: CellIndex) {
        let mut drawing = DrawingState::new();
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
        let drawing = DrawingState::new();
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
