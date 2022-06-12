use crate::drawing::assets::PIXELS_PER_TILE_WIDTH;
use crate::drawing::coords::cast::Cast;
use crate::drawing::{assets, Drawing, SubTilePosition, TilePosition};
use crate::input::PixelPosition;

pub fn tile_to_pixel(tile: TilePosition, drawing: &Drawing, screen_width: f32) -> PixelPosition {
    let subtile = SubTilePosition::new(tile.x as f32, tile.y as f32);
    subtile_to_pixel(subtile, drawing, screen_width)
}

pub fn subtile_to_pixel(
    tile: SubTilePosition,
    drawing: &Drawing,
    screen_width: f32,
) -> PixelPosition {
    let offset = pixel_offset(drawing, screen_width);
    let pixel = subtile_to_pixel_offset(tile) + offset;
    pixel
}

#[allow(dead_code)]
pub fn pixel_to_tile(
    pixel_position: PixelPosition,
    drawing: &Drawing,
    screen_width: f32,
) -> TilePosition {
    let offset = pixel_offset(drawing, screen_width);
    let tile = pixel_to_tile_offset(pixel_position - offset);
    tile
}
pub fn pixel_to_subtile(
    pixel_position: PixelPosition,
    drawing: &Drawing,
    screen_width: f32,
) -> SubTilePosition {
    let offset = pixel_offset(drawing, screen_width);
    let subtile = pixel_to_subtile_offset(pixel_position - offset);
    subtile
}

pub fn pixel_offset(drawing: &Drawing, screen_width: f32) -> PixelPosition {
    let center_tile = PIXELS_PER_TILE_WIDTH as f32 * 0.5;
    let screen_center = screen_width / 2.0;
    let pixels_subtile_offset = subtile_to_pixel_offset(drawing.subtile_offset);
    PixelPosition::new(screen_center - center_tile, 0.0) + pixels_subtile_offset
}

pub fn subtile_to_pixel_offset(subtile: SubTilePosition) -> PixelPosition {
    PixelPosition::new(
        subtile.x * (assets::PIXELS_PER_TILE_WIDTH as f32 * 0.5),
        subtile.y * (assets::PIXELS_PER_TILE_HEIGHT as f32 * 0.25),
    )
}

pub fn pixel_to_subtile_offset(pixel_diff: PixelPosition) -> SubTilePosition {
    SubTilePosition::new(
        pixel_diff.x / (assets::PIXELS_PER_TILE_WIDTH as f32 * 0.5),
        pixel_diff.y / (assets::PIXELS_PER_TILE_HEIGHT as f32 * 0.25),
    )
}

#[allow(dead_code)]
pub fn pixel_to_tile_offset(pixel_diff: PixelPosition) -> TilePosition {
    pixel_to_subtile_offset(pixel_diff).cast()
}

#[cfg(test)]
mod tests {
    use super::*;

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
}
