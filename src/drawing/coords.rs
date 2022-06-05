use crate::drawing::assets::{PIXELS_PER_TILE_HEIGHT, PIXELS_PER_TILE_WIDTH};
use crate::drawing::{
    pixel_to_cell_offset_2, subtile_to_subcell_offset, tile_to_cell_offset, Drawing, PixelPosition,
    SubCellIndex, SubTilePosition, TilePosition,
};
use crate::map::CellIndex;
use crate::DrawingTrait;
use std::cmp::max;

pub fn cell_to_pixel(cell_index: CellIndex, drawing: &Drawing, screen_width: f32) -> PixelPosition {
    let tile = cell_to_tile(
        &drawing.min_cell,
        &drawing.max_cell,
        cell_index.x,
        cell_index.y,
        cell_index.z,
    );
    tile_to_pixel(tile, drawing, screen_width)
}

pub fn cell_to_tile(
    min_cell: &CellIndex,
    max_cell: &CellIndex,
    i_x: i32,
    i_y: i32,
    i_z: i32,
) -> TilePosition {
    TilePosition::new(
        ((i_x - min_cell.x) - (i_z - min_cell.z)),
        ((i_x - min_cell.x) + (i_z - min_cell.z) + 2 * (max_cell.y - i_y)),
    )
}

pub fn subcell_to_subtile(
    subcell: SubCellIndex,
    min_cell: &CellIndex,
    max_cell: &CellIndex,
) -> SubTilePosition {
    // NOTE we are mixing min_cell and max_cell !!! this is intended
    let subcell_offset =
        subcell - SubCellIndex::new(min_cell.x as f32, max_cell.y as f32, min_cell.z as f32);
    SubTilePosition::new(
        subcell_offset.x - subcell_offset.z,
        subcell_offset.x + subcell_offset.z - 2.0 * subcell_offset.y,
    )
}

pub fn tile_to_cell(tile: TilePosition, min_cell: &CellIndex, max_cell: &CellIndex) -> CellIndex {
    let mut cell_offset = tile_to_cell_offset(tile);
    cell_offset.x += min_cell.x;
    cell_offset.y = max_cell.y;
    cell_offset.z += min_cell.z;
    cell_offset
}

pub fn tile_to_pixel(tile: TilePosition, drawing: &Drawing, screen_width: f32) -> PixelPosition {
    subtile_to_pixel(
        SubTilePosition::new(tile.x as f32, tile.y as f32),
        drawing,
        screen_width,
    )
}
pub fn subtile_to_pixel(
    tile: SubTilePosition,
    drawing: &Drawing,
    screen_width: f32,
) -> PixelPosition {
    let pixels_half_tile_x = PIXELS_PER_TILE_WIDTH as f32 * 0.5;
    let pixels_half_tile_y = PIXELS_PER_TILE_HEIGHT as f32 * 0.5;
    let pixels_height_isometric = pixels_half_tile_y * 0.5;
    let x = f32::trunc(
        tile.x * pixels_half_tile_x + screen_width / 2.0 - pixels_half_tile_x
            + drawing.subtile_offset.x * PIXELS_PER_TILE_WIDTH as f32,
    );
    let y = f32::trunc(
        tile.y * pixels_height_isometric + drawing.subtile_offset.y * PIXELS_PER_TILE_HEIGHT as f32,
    );
    PixelPosition::new(x, y)
}

pub fn pixel_to_subtile(
    pixel_position: PixelPosition,
    drawing: &Drawing,
    screen_width: f32,
) -> SubTilePosition {
    let pixels_half_tile_x = PIXELS_PER_TILE_WIDTH as f32 * 0.5;
    let pixels_half_tile_y = PIXELS_PER_TILE_HEIGHT as f32 * 0.5;
    let pixels_height_isometric = pixels_half_tile_y * 0.5;
    let tile_x = (pixel_position.x
        - (screen_width / 2.0 - pixels_half_tile_x
            + drawing.subtile_offset.x * PIXELS_PER_TILE_WIDTH as f32))
        / pixels_half_tile_x;
    let tile_y = (pixel_position.y - (drawing.subtile_offset.y * PIXELS_PER_TILE_HEIGHT as f32))
        / pixels_height_isometric;
    SubTilePosition::new(tile_x, tile_y)
}

pub fn pixel_to_cell(
    pixel_position: PixelPosition,
    drawing: &Drawing,
    screen_width: f32,
) -> CellIndex {
    let subtile_offset = pixel_to_subtile(pixel_position, drawing, screen_width);
    let cell_index = tile_to_cell(
        TilePosition::new(subtile_offset.x as i32, subtile_offset.y as i32),
        &drawing.min_cell,
        &drawing.max_cell,
    );
    cell_index
}
pub fn pixel_to_subcell_center(
    pixel: PixelPosition,
    drawing: &Drawing,
    screen_width: f32,
) -> SubCellIndex {
    let mut subtile = pixel_to_subtile(pixel, drawing, screen_width);

    // move the hitbox to the center of the tile
    let subtile_center = SubTilePosition::new(subtile.x - 0.25, subtile.y - 0.25);
    let mut cell_offset = subtile_to_subcell_offset(subtile_center);
    let fractional_part_y = cell_offset.y - f32::floor(cell_offset.y);
    cell_offset.x += drawing.min_cell.x as f32;
    cell_offset.y = drawing.max_cell.y as f32 + fractional_part_y;
    cell_offset.z += drawing.min_cell.z as f32;
    cell_offset
}

pub fn subcell_center_to_pixel(
    subcell: SubCellIndex,
    drawing: &Drawing,
    screen_width: f32,
) -> PixelPosition {
    let subtile_center = subcell_to_subtile(subcell, &drawing.min_cell, &drawing.max_cell);

    let subtile = SubTilePosition::new(subtile_center.x + 0.25, subtile_center.y + 0.25);
    subtile_to_pixel(subtile, drawing, screen_width)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::drawing::{tile_to_cell_offset, TilePosition};
    use crate::IVec3;

    #[test]
    fn position_tile_basic() {
        let min_cell = CellIndex::new(0, 0, 0);
        let max_cell = CellIndex::new(10, 10, 10);
        assert_eq!(
            cell_to_tile(&min_cell, &max_cell, 0, max_cell.y, 0),
            TilePosition::new(0, 0)
        );
        assert_eq!(
            cell_to_tile(&min_cell, &max_cell, 1, max_cell.y, 1),
            TilePosition::new(0, 2)
        );
    }

    #[test]
    fn position_tile_min_cell() {
        let min_cell = CellIndex::new(0, 0, 0);
        let max_cell = CellIndex::new(10, 10, 10);
        let tile = cell_to_tile(&min_cell, &max_cell, min_cell.x, max_cell.y, min_cell.z);
        assert_eq!(tile, TilePosition::new(0, 0));
    }

    #[test]
    fn position_tile_negative() {
        let min_cell = CellIndex::new(-5, -25, -55);
        let max_cell = CellIndex::new(5, -15, -45);
        assert_eq!(
            cell_to_tile(&min_cell, &max_cell, min_cell.x, max_cell.y, min_cell.z),
            TilePosition::new(0, 0)
        );
        assert_eq!(
            cell_to_tile(
                &min_cell,
                &max_cell,
                min_cell.x + 1,
                max_cell.y,
                min_cell.z + 1
            ),
            TilePosition::new(0, 2)
        );
    }

    #[test]
    fn position_tile_height() {
        let min_cell = CellIndex::new(-5, -25, -55);
        let max_cell = CellIndex::new(5, -15, -45);
        assert_eq!(
            cell_to_tile(
                &min_cell,
                &max_cell,
                min_cell.x + 1,
                max_cell.y,
                min_cell.z + 1
            ),
            cell_to_tile(&min_cell, &max_cell, min_cell.x, max_cell.y - 1, min_cell.z)
        );
    }

    fn cell_to_tile_to_cell(initial_cell: CellIndex) {
        let min_cell = CellIndex::new(-10, -10, -10);
        let max_cell = CellIndex::new(10, initial_cell.y, 10);
        let tile = cell_to_tile(
            &min_cell,
            &max_cell,
            initial_cell.x,
            initial_cell.y,
            initial_cell.z,
        );
        let final_cell = tile_to_cell(tile, &min_cell, &max_cell);
        assert_eq!(final_cell, initial_cell);
    }
    #[test]
    fn test_cell_to_tile_to_cell() {
        cell_to_tile_to_cell(CellIndex::new(0, 0, 0));
        cell_to_tile_to_cell(CellIndex::new(1, 0, 0));
        cell_to_tile_to_cell(CellIndex::new(0, 1, 0));
        cell_to_tile_to_cell(CellIndex::new(0, 0, 1));
    }

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

    fn tile_to_pixel_to_tile(initial_tile: TilePosition) {
        let mut drawing = Drawing::new();
        let pixel = tile_to_pixel(initial_tile, &drawing, 800.0);
        let final_subtile = pixel_to_subtile(pixel, &drawing, 800.0);
        let intial_subtile = SubTilePosition::new(initial_tile.x as f32, initial_tile.y as f32);
        assert_eq!(final_subtile, intial_subtile);
    }

    #[test]
    fn test_tile_to_pixel_to_tile() {
        tile_to_pixel_to_tile(TilePosition::new(0, 0));
        tile_to_pixel_to_tile(TilePosition::new(1, 0));
        tile_to_pixel_to_tile(TilePosition::new(0, 1));
        tile_to_pixel_to_tile(TilePosition::new(1, 1));
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
