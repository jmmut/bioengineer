pub mod assets;

use crate::game_state::GameState;
use crate::map::trunc::{trunc_towards_neg_inf, trunc_towards_neg_inf_f};
use crate::map::{CellIndex, Map, TileType};
use crate::{input, Color, IVec2, IVec3, Texture2D, Vec2, Vec3};
use assets::{PIXELS_PER_TILE_HEIGHT, PIXELS_PER_TILE_WIDTH};
use input::Input;

pub type PixelPosition = Vec2;
pub type TilePosition = IVec2;
pub type SubTilePosition = Vec2;
pub type SubCellIndex = Vec3;
const GREY: Color = Color::new(0.5, 0.5, 0.5, 1.0);
const BLACK: Color = Color::new(0.0, 0.0, 0.0, 1.0);
const FONT_SIZE: f32 = 20.0;

pub struct Drawing {
    min_cell: CellIndex,
    max_cell: CellIndex,
    subtile_offset: PixelPosition,
}

impl Drawing {
    pub fn new() -> Self {
        Drawing {
            min_cell: CellIndex::new(-8, 0, -8),
            max_cell: CellIndex::new(7, 2, 7),
            subtile_offset: PixelPosition::new(0.0, 0.0),
        }
    }
}

pub trait DrawingTrait {
    fn new(textures: Vec<Texture2D>) -> Self;
    fn apply_input(&mut self, input: &Input) {
        self.change_height_rel(input.change_height_rel);
        self.move_map_horizontally(input.move_map_horizontally);
    }
    fn draw(&self, game_state: &GameState) {
        self.clear_background(GREY);
        self.draw_map(game_state);
        self.draw_fps(game_state);
        self.draw_level(self.drawing().min_cell.y, self.drawing().max_cell.y);
    }

    fn draw_fps(&self, game_state: &GameState) {
        let fps = 1.0 / (game_state.current_frame_ts - game_state.previous_frame_ts);
        // println!(
        //     "now - previous ts: {} - {}, fps: {}, frame: {}",
        //     game_state.current_frame_ts, game_state.previous_frame_ts, fps, game_state.frame_index
        // );
        let text = format!("{:.0}", fps);
        self.draw_text(
            text.as_str(),
            self.screen_width() - FONT_SIZE * 2.0,
            20.0,
            FONT_SIZE,
            BLACK,
        );
    }
    fn draw_level(&self, min_y: i32, max_y: i32) {
        let text = format!("height: [{}, {}]", min_y, max_y);
        self.draw_text(
            text.as_str(),
            20.0,
            self.screen_height() - FONT_SIZE * 1.0,
            FONT_SIZE,
            BLACK,
        );
    }
    fn draw_texture(&self, tile: TileType, x: f32, y: f32);
    fn draw_transparent_texture(&self, tile: TileType, x: f32, y: f32, opacity_coef: f32);
    fn clear_background(&self, color: Color);
    fn drawing(&self) -> &Drawing;
    fn drawing_mut(&mut self) -> &mut Drawing;
    fn screen_width(&self) -> f32;
    fn screen_height(&self) -> f32;
    fn draw_text(&self, text: &str, x: f32, y: f32, font_size: f32, color: Color);
    fn change_height_rel(&mut self, y: i32) {
        if y != 0 {
            let drawing_ = self.drawing_mut();
            let min_cell = &mut drawing_.min_cell;
            let max_cell = &mut drawing_.max_cell;
            max_cell.y += y;
            min_cell.y += y;
            if min_cell.y < Map::min_cell().y {
                let diff = Map::min_cell().y - min_cell.y;
                min_cell.y += diff;
                max_cell.y += diff;
            } else if max_cell.y > Map::max_cell().y {
                let diff = Map::max_cell().y - max_cell.y;
                min_cell.y += diff;
                max_cell.y += diff;
            }
        }
    }
    fn move_map_horizontally(&mut self, diff: PixelPosition) {
        let mut tile_offset = TilePosition::new(0, 0);
        let drawing_ = self.drawing_mut();
        if diff.x != 0.0 {
            (tile_offset.x, drawing_.subtile_offset.x) = pixel_to_tile_offset(
                diff.x,
                drawing_.subtile_offset.x,
                assets::PIXELS_PER_TILE_WIDTH as f32,
            );
        }
        if diff.y != 0.0 {
            (tile_offset.y, drawing_.subtile_offset.y) = pixel_to_tile_offset(
                diff.y,
                drawing_.subtile_offset.y,
                assets::PIXELS_PER_TILE_WIDTH as f32 * 0.5,
            );
        }
        print!("subtile_offset: {}\r", drawing_.subtile_offset);
        if tile_offset.x != 0 || tile_offset.y != 0 {
            let min_cell = &mut drawing_.min_cell;
            let max_cell = &mut drawing_.max_cell;
            let cell_diff = tile_to_cell_offset(tile_offset);

            // TODO: disallow sub-tile offset if already on a min_cell or max_cell
            max_cell.x -= cell_diff.x;
            min_cell.x -= cell_diff.x;
            max_cell.z -= cell_diff.z;
            min_cell.z -= cell_diff.z;
            if min_cell.x < Map::min_cell().x {
                let diff = Map::min_cell().x - min_cell.x;
                min_cell.x += diff;
                max_cell.x += diff;
            } else if max_cell.x > Map::max_cell().x {
                let diff = Map::max_cell().x - max_cell.x;
                min_cell.x += diff;
                max_cell.x += diff;
            }
            if min_cell.z < Map::min_cell().z {
                let diff = Map::min_cell().z - min_cell.z;
                min_cell.z += diff;
                max_cell.z += diff;
            } else if max_cell.z > Map::max_cell().z {
                let diff = Map::max_cell().z - max_cell.z;
                min_cell.z += diff;
                max_cell.z += diff;
            }
        }
    }

    fn draw_map(&self, game_state: &GameState) {
        let min_cell = &self.drawing().min_cell;
        let max_cell = &self.drawing().max_cell;
        for i_y in min_cell.y..=max_cell.y {
            for i_z in min_cell.z..=max_cell.z {
                for i_x in min_cell.x..=max_cell.x {
                    let cell_index = CellIndex::new(i_x, i_y, i_z);
                    let (x, y) = self.get_draw_position(i_x, i_y, i_z);
                    let opacity = get_opacity(
                        &cell_index,
                        &min_cell,
                        &max_cell,
                        self.drawing().subtile_offset,
                    );
                    self.draw_transparent_texture(
                        game_state.map.get_cell(cell_index).tile_type,
                        x,
                        y,
                        opacity,
                    );
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
        x = f32::trunc(
            x * pixels_half_tile_x + self.screen_width() / 2.0 - pixels_half_tile_x
                + self.drawing().subtile_offset.x * PIXELS_PER_TILE_WIDTH as f32,
        );
        y = f32::trunc(
            y * pixels_height_isometric
                + self.drawing().subtile_offset.y * PIXELS_PER_TILE_HEIGHT as f32 * 0.5,
        );
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
/// returns the integer and decimal part of the offset
fn pixel_to_tile_offset(
    new_pixel_offset: f32,
    existing_subtile_offset: f32,
    tile_size: f32,
) -> (i32, f32) {
    let new_tile_offset = (new_pixel_offset + existing_subtile_offset * tile_size) / tile_size;
    let int_tile_offset = trunc_towards_neg_inf_f(new_tile_offset);
    let new_subtiles_offset = new_tile_offset - int_tile_offset;
    assert_in_range_0_1(new_subtiles_offset);
    (int_tile_offset as i32, new_subtiles_offset)
}

fn tile_to_cell_offset(tile_offset: TilePosition) -> CellIndex {
    CellIndex::new(
        tile_offset.x + tile_offset.y,
        0,
        -tile_offset.x + tile_offset.y,
    )
}
fn subtile_to_subcell_offset(subtile_offset: SubTilePosition) -> SubCellIndex {
    SubCellIndex::new(
        subtile_offset.x + subtile_offset.y,
        0.0,
        -subtile_offset.x + subtile_offset.y,
    )
}

fn get_opacity(
    cell: &CellIndex,
    min_cell: &CellIndex,
    max_cell: &CellIndex,
    subtile_offset: PixelPosition,
) -> f32 {
    if cell.x == min_cell.x {
        assert_in_range_0_1(
            // 1.0,
            (subtile_offset.x + subtile_offset.y) *0.5
        )
    } else if cell.x == max_cell.x {
        assert_in_range_0_1(
            // 1.0,
            1.0 - (subtile_offset.x + subtile_offset.y) * 0.5
                // 1.0 - (drawing_offset.x - drawing_offset.y)/assets::PIXELS_PER_TILE_WIDTH as f32 * 0.5
        )
    } else if cell.z == min_cell.z {
        assert_in_range_0_1(
            // 1.0,
            (-subtile_offset.x + subtile_offset.y) *0.5 +0.5
        )
    } else if cell.z == max_cell.z {
        assert_in_range_0_1(
         1.0 - ((-subtile_offset.x + subtile_offset.y) * 0.5 + 0.5)
        )
    } else {
        1.0
    }
}

fn assert_in_range_0_1(x: f32) -> f32 {
    if x < 0.0 || x > 1.0 {
        panic!("out of range: {}", x);
    } else {
        x
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::IVec3;

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

    #[test]
    fn transparency_border() {
        let min_cell = CellIndex::new(-5, -25, -55);
        let max_cell = CellIndex::new(5, -15, -45);
        let mut cell = CellIndex::new(0, 0, 0);
        let no_offset = PixelPosition::new(0.0, 0.0);
        let t = get_opacity(&cell, &min_cell, &max_cell, no_offset);
        assert_eq!(t, 1.0);

        cell.x = min_cell.x;
        let t = get_opacity(&cell, &min_cell, &max_cell, no_offset);
        assert_eq!(t, 0.0);
    }
}
