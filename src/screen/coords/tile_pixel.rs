use crate::screen::assets;
use crate::screen::assets::PIXELS_PER_TILE_WIDTH;
use crate::screen::coords::cast::Cast;
use crate::screen::drawing_state::{DrawingState, SubTilePosition, TilePosition};
use crate::screen::input::PixelPosition;

pub fn tile_to_pixel(
    tile: TilePosition,
    drawing: &DrawingState,
    screen_width: f32,
) -> PixelPosition {
    let subtile = SubTilePosition::new(tile.x as f32, tile.y as f32);
    subtile_to_pixel(subtile, drawing, screen_width)
}

pub fn subtile_to_pixel(
    tile: SubTilePosition,
    drawing: &DrawingState,
    screen_width: f32,
) -> PixelPosition {
    let offset = pixel_offset(drawing, screen_width);
    let pixel = subtile_to_pixel_offset(tile, drawing.zoom) + offset;
    pixel
}

#[allow(dead_code)]
pub fn pixel_to_tile(
    pixel_position: PixelPosition,
    drawing: &DrawingState,
    screen_width: f32,
) -> TilePosition {
    let offset = pixel_offset(drawing, screen_width);
    let tile = pixel_to_tile_offset(pixel_position - offset, drawing.zoom);
    tile
}
pub fn pixel_to_subtile(
    pixel_position: PixelPosition,
    drawing: &DrawingState,
    screen_width: f32,
) -> SubTilePosition {
    let offset = pixel_offset(drawing, screen_width);
    let subtile = pixel_to_subtile_offset(pixel_position - offset, drawing.zoom);
    subtile
}

pub fn pixel_offset(drawing: &DrawingState, screen_width: f32) -> PixelPosition {
    let center_tile = PIXELS_PER_TILE_WIDTH as f32 * 0.5;
    let screen_center = screen_width / 2.0;
    let pixels_subtile_offset = subtile_to_pixel_offset(drawing.subtile_offset, drawing.zoom);
    PixelPosition::new(screen_center - center_tile, 0.0) + pixels_subtile_offset
}

pub fn subtile_to_pixel_offset(subtile: SubTilePosition, zoom: f32) -> PixelPosition {
    PixelPosition::new(
        subtile.x * (assets::PIXELS_PER_TILE_WIDTH as f32 * 0.5 * zoom),
        subtile.y * (assets::PIXELS_PER_TILE_HEIGHT as f32 * 0.25 * zoom),
    )
}

pub fn pixel_to_subtile_offset(pixel_diff: PixelPosition, zoom: f32) -> SubTilePosition {
    SubTilePosition::new(
        pixel_diff.x / (assets::PIXELS_PER_TILE_WIDTH as f32 * 0.5 * zoom),
        pixel_diff.y / (assets::PIXELS_PER_TILE_HEIGHT as f32 * 0.25 * zoom),
    )
}

#[allow(dead_code)]
pub fn pixel_to_tile_offset(pixel_diff: PixelPosition, zoom: f32) -> TilePosition {
    pixel_to_subtile_offset(pixel_diff, zoom).cast()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tile_to_pixel_to_tile() {
        tile_to_pixel_to_tile(TilePosition::new(0, 0));
        tile_to_pixel_to_tile(TilePosition::new(1, 0));
        tile_to_pixel_to_tile(TilePosition::new(0, 1));
        tile_to_pixel_to_tile(TilePosition::new(1, 1));
    }

    #[test]
    fn test_zoom() {
        let mut drawing = DrawingState::new();
        drawing.zoom = 2.0;
        tile_to_pixel_to_tile(TilePosition::new(0, 0));
        tile_to_pixel_to_tile(TilePosition::new(1, 0));
        tile_to_pixel_to_tile(TilePosition::new(0, 1));
        tile_to_pixel_to_tile(TilePosition::new(1, 1));
    }

    fn tile_to_pixel_to_tile(initial_tile: TilePosition) {
        let drawing = DrawingState::new();
        tile_to_pixel_to_tile_with_drawing(initial_tile, drawing);
    }

    fn tile_to_pixel_to_tile_with_drawing(initial_tile: TilePosition, drawing: DrawingState) {
        let pixel = tile_to_pixel(initial_tile, &drawing, 800.0);
        let final_subtile = pixel_to_subtile(pixel, &drawing, 800.0);
        let intial_subtile = SubTilePosition::new(initial_tile.x as f32, initial_tile.y as f32);
        assert_eq!(final_subtile, intial_subtile);
    }
}
