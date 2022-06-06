pub mod assets;
mod coords;
mod hud;
mod tiles;

use crate::drawing::coords::cast::Cast;
use crate::drawing::coords::cell_pixel::{cell_to_pixel, pixel_to_subcell_offset};
use crate::drawing::coords::cell_tile::subcell_to_subtile_offset;
use crate::drawing::coords::truncate::truncate_cell_offset;
use crate::drawing::tiles::{hitbox_offset, hitbox_offset_square};
use crate::game_state::GameState;
use crate::map::trunc::{trunc_towards_neg_inf, trunc_towards_neg_inf_f};
use crate::map::{Cell, CellCubeIterator, CellIndex, Map, TileType};
use crate::{input, Color, IVec2, IVec3, Texture2D, Vec2, Vec3};
use assets::{PIXELS_PER_TILE_HEIGHT, PIXELS_PER_TILE_WIDTH};
use coords::cell_pixel;
use coords::cell_pixel::{
    pixel_to_cell, pixel_to_subcell, pixel_to_subcell_center, subcell_center_to_pixel,
};
use input::Input;
use macroquad::shapes::draw_rectangle;
use std::cmp::min;
use std::collections::HashSet;

pub type PixelPosition = Vec2;
pub type TilePosition = IVec2;
pub type SubTilePosition = Vec2;
pub type SubCellIndex = Vec3;
const GREY: Color = Color::new(0.5, 0.5, 0.5, 1.0);

pub fn apply_input(drawer: &mut impl DrawingTrait, input: &Input) {
    let screen_width = drawer.screen_width();
    let drawing = drawer.drawing_mut();
    drawing.change_height_rel(input.change_height_rel);
    drawing.move_map_horizontally(input.move_map_horizontally, screen_width);
    drawing.select_cells(
        input.start_selection,
        input.end_selection,
        input.mouse_position,
        screen_width,
    );
}

pub fn draw(drawer: &impl DrawingTrait, game_state: &GameState) {
    drawer.clear_background(GREY);
    tiles::draw_map(drawer, game_state);
    hud::draw_fps(drawer, game_state);
    hud::draw_level(
        drawer,
        drawer.drawing().min_cell.y,
        drawer.drawing().max_cell.y,
    );
}

pub struct Drawing {
    min_cell: CellIndex,
    max_cell: CellIndex,
    subtile_offset: SubTilePosition,
    subcell_diff: SubCellIndex,
    highlight_start: Option<CellIndex>,
    highlight_end: Option<CellIndex>,
    highlighted_cells: HashSet<CellIndex>,
}

impl Drawing {
    pub fn new() -> Self {
        Drawing {
            min_cell: CellIndex::new(-10, -1, -10),
            max_cell: CellIndex::new(9, 1, 9),
            subtile_offset: SubTilePosition::new(0.0, 0.0),
            subcell_diff: SubCellIndex::new(0.0, 0.0, 0.0),
            highlight_start: Option::None,
            highlight_end: Option::None,
            highlighted_cells: HashSet::new(),
        }
    }

    fn change_height_rel(&mut self, y: i32) {
        if y != 0 {
            let drawing_ = self;
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

    fn move_map_horizontally(&mut self, diff: PixelPosition, screen_width: f32) {
        if diff == PixelPosition::new(0.0, 0.0) {
            return;
        }
        let drawing_ = self;
        let subcell_diff_ = pixel_to_subcell_offset(diff);

        // let new_cell_offset = pixel_to_cell_offset(diff);

        // println!(
        //     "pixel_diff: {}, subcell_diff: {}",
        //     diff, subcell_diff_
        // );
        let (truncated_cell_diff, truncated_subcell_diff) =
            truncate_cell_offset(subcell_diff_ + drawing_.subcell_diff);
        drawing_.subcell_diff = truncated_subcell_diff;

        // println!(
        //     "truncated_cell_diff: {}, truncated_subcell_diff: {}",
        //     truncated_cell_diff, truncated_subcell_diff
        // );
        let min_cell = &mut drawing_.min_cell;
        let max_cell = &mut drawing_.max_cell;

        max_cell.x -= truncated_cell_diff.x;
        min_cell.x -= truncated_cell_diff.x;
        max_cell.z -= truncated_cell_diff.z;
        min_cell.z -= truncated_cell_diff.z;
        if min_cell.x <= Map::min_cell().x {
            let diff = Map::min_cell().x - min_cell.x;
            // println!("outside of map! resetting subtile_offset and subcell_diff.");
            // print!("min_cell from {}", min_cell);
            min_cell.x += diff;
            max_cell.x += diff;
            // println!(" to {}", min_cell);
            drawing_.subcell_diff.x = 0.0;
        } else if max_cell.x > Map::max_cell().x {
            let diff = Map::max_cell().x - max_cell.x;
            min_cell.x += diff;
            max_cell.x += diff;
            drawing_.subcell_diff.x = 0.0;
        }
        if min_cell.z <= Map::min_cell().z {
            let diff = Map::min_cell().z - min_cell.z;
            min_cell.z += diff;
            max_cell.z += diff;
            drawing_.subcell_diff.z = 0.0;
        } else if max_cell.z > Map::max_cell().z {
            let diff = Map::max_cell().z - max_cell.z;
            min_cell.z += diff;
            max_cell.z += diff;
            drawing_.subcell_diff.z = 0.0;
        }

        drawing_.subtile_offset = subcell_to_subtile_offset(drawing_.subcell_diff);
        // {
        //     let test_cell = CellIndex::new(2, drawing_.max_cell.y, 2);
        //     let p = cell_to_pixel(test_cell, drawing_, screen_width);
        //     let test_cell_2 = pixel_to_cell(p, drawing_, screen_width);
        //     println!("for test_cell {}, got cell {}", test_cell, test_cell_2);
        // }
        // println!("subtile_offset: {}\n ", drawing_.subtile_offset);
    }

    fn select_cells(
        &mut self,
        start_selection: Option<PixelPosition>,
        end_selection: Option<PixelPosition>,
        mouse_position: PixelPosition,
        screen_width: f32,
    ) {
        let drawing_ = self;
        match start_selection {
            None => {}
            Some(selected) => {
                let moved_selected = selected + hitbox_offset();
                let subcell = pixel_to_subcell_center(moved_selected, drawing_, screen_width);
                let (cell, _) = truncate_cell_offset(subcell);
                drawing_.highlight_start = Option::Some(cell);
                drawing_.highlight_end = Option::None;
            }
        }
        if drawing_.highlight_start.is_some() {
            match drawing_.highlight_end {
                None => {
                    let moved_selected = mouse_position + hitbox_offset();
                    let end_subcell = pixel_to_subcell_center(moved_selected, drawing_,
                                                             screen_width);
                    let (end_cell, _) = truncate_cell_offset(end_subcell);
                    let cell_cube = CellCubeIterator::new_from_mixed(
                        drawing_.highlight_start.unwrap(),
                        end_cell,
                    );
                    drawing_.highlighted_cells.clear();
                    for cell in cell_cube {
                        drawing_.highlighted_cells.insert(cell);
                    }
                }
                Some(_) => {}
            }
        }
        match end_selection {
            None => {}
            Some(selected) => {
                let moved_selected = selected + hitbox_offset();
                let end_subcell = pixel_to_subcell_center(moved_selected, drawing_, screen_width);
                let (end_cell, _) = truncate_cell_offset(end_subcell);
                drawing_.highlight_end = Option::Some(end_cell);
            }
        }

                // let subcell = pixel_to_subcell_center(moved_selected, self, screen_width);
                // drawing_.highlighted_cells.clear();
                // // let local_cell_index = pixel_to_cell_offset(selected, ).0;
                // // let global_cell_index = local_cell_index + drawing_.min_cell;
                // // println!("min cell {}", drawing_.min_cell);
                // // println!("local cell {}", local_cell_index);
                // // println!("selected cell {}", subcell);
                // let (cell, _) = truncate_cell_offset(subcell);
                // // println!("selected truncated cell {}", cell);
                // drawing_.highlighted_cells.insert(cell);

    }
    fn end_select_cell(&mut self, end_selection: Option<PixelPosition>, screen_width: f32) {
        match end_selection {
            None => {}
            Some(selected) => {
                let moved_selected = selected + hitbox_offset();
                let subcell = pixel_to_subcell_center(moved_selected, self, screen_width);
                let drawing_ = self;
                drawing_.highlighted_cells.clear();
                // let local_cell_index = pixel_to_cell_offset(selected, ).0;
                // let global_cell_index = local_cell_index + drawing_.min_cell;
                // println!("min cell {}", drawing_.min_cell);
                // println!("local cell {}", local_cell_index);
                // println!("selected cell {}", subcell);
                let (cell, _) = truncate_cell_offset(subcell);
                // println!("selected truncated cell {}", cell);
                drawing_.highlighted_cells.insert(cell);
            }
        }
    }
}

/// Trait to be implemented by a graphics library.
/// The purpose of this class is to decouple the project from the graphics library.
/// Hopefully, if I ever need to swap the graphics library (currently macroquad), classes like
/// this one will be the only places to change.
/// I'm not sure this will actually help, but we'll see.
pub trait DrawingTrait {
    fn new(textures: Vec<Texture2D>) -> Self
    where
        Self: Sized;

    fn screen_width(&self) -> f32;
    fn screen_height(&self) -> f32;
    fn clear_background(&self, color: Color);
    fn draw_texture(&self, tile: TileType, x: f32, y: f32);
    fn draw_transparent_texture(&self, tile: TileType, x: f32, y: f32, opacity_coef: f32);
    fn draw_colored_texture(&self, tile: TileType, x: f32, y: f32, color_mask: Color);
    fn draw_rectangle(&self, x: f32, y: f32, w: f32, h: f32, color: Color);
    fn draw_text(&self, text: &str, x: f32, y: f32, font_size: f32, color: Color);
    fn drawing(&self) -> &Drawing;
    fn drawing_mut(&mut self) -> &mut Drawing;
}
