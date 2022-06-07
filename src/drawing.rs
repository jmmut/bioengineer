pub mod assets;
mod coords;
mod hud;
mod tiles;

use crate::drawing::coords::cell_pixel::clicked_cell;
use crate::game_state::GameState;
use crate::input::CellSelection::{NoSelection, SelectionFinished, SelectionStarted};
use crate::input::{CellSelection, Input, PixelPosition};
use crate::map::trunc::{trunc_towards_neg_inf, trunc_towards_neg_inf_f};
use crate::map::{Cell, CellCubeIterator, CellIndex, Map, TileType};
use crate::{Color, IVec2, IVec3, Texture2D, Vec2, Vec3};
use assets::{PIXELS_PER_TILE_HEIGHT, PIXELS_PER_TILE_WIDTH};
use coords::cast::Cast;
use coords::cell_pixel;
use coords::cell_pixel::{cell_to_pixel, pixel_to_subcell_offset};
use coords::cell_pixel::{
    pixel_to_cell, pixel_to_subcell, pixel_to_subcell_center, subcell_center_to_pixel,
};
use coords::cell_tile::subcell_to_subtile_offset;
use coords::truncate::truncate_cell_offset;
use std::cmp::min;
use std::collections::HashSet;
use std::io::SeekFrom::Start;
use tiles::{hitbox_offset, hitbox_offset_square};

pub type TilePosition = IVec2;
pub type SubTilePosition = Vec2;
pub type SubCellIndex = Vec3;
const GREY: Color = Color::new(0.5, 0.5, 0.5, 1.0);

pub fn apply_input(drawer: &mut impl DrawingTrait, input: &Input) {
    let screen_width = drawer.screen_width();
    let drawing = drawer.drawing_mut();
    drawing.change_height_rel(input.change_height_rel);
    drawing.move_map_horizontally(input.move_map_horizontally, screen_width);
    drawing.select_cells(&input.cell_selection, screen_width);
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
            move_inside_range(
                &mut min_cell.y,
                &mut max_cell.y,
                Map::min_cell().y,
                Map::max_cell().y,
            );
        }
    }

    fn move_map_horizontally(&mut self, diff: PixelPosition, screen_width: f32) {
        if diff == PixelPosition::new(0.0, 0.0) {
            return;
        }
        let mut drawing_ = self;
        let subcell_diff_ = pixel_to_subcell_offset(diff);

        // let new_cell_offset = pixel_to_cell_offset(diff);

        // println!(
        //     "pixel_diff: {}, subcell_diff: {}",
        //     diff, subcell_diff_
        // );
        let (truncated_cell_diff, truncated_subcell_diff) =
            truncate_cell_offset(subcell_diff_ + drawing_.subcell_diff);

        move_map_horizontally_to(&mut drawing_, truncated_cell_diff, truncated_subcell_diff);
    }

    fn select_cells(&mut self, cell_selection: &CellSelection, screen_width: f32) {
        match cell_selection {
            NoSelection => (),
            SelectionStarted(cell_selection) => {
                let (start_cell, _) = highlight_cells_from_pixels(
                    cell_selection.start,
                    cell_selection.end,
                    screen_width,
                    self,
                );
                self.highlight_start = Option::Some(start_cell);
                self.highlight_end = Option::None;
            }
            SelectionFinished(cell_selection) => {
                let (start_cell, end_cell) = highlight_cells_from_pixels(
                    cell_selection.start,
                    cell_selection.end,
                    screen_width,
                    self,
                );
                self.highlight_start = Option::Some(start_cell);
                self.highlight_end = Option::Some(end_cell);
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

fn move_map_horizontally_to(
    drawing_: &mut Drawing,
    truncated_cell_diff: CellIndex,
    truncated_subcell_diff: SubCellIndex,
) {
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
    if move_inside_range_min_equals(
        &mut min_cell.x,
        &mut max_cell.x,
        Map::min_cell().x,
        Map::max_cell().x,
    ) {
        drawing_.subcell_diff.x = 0.0;
    }
    if move_inside_range_min_equals(
        &mut min_cell.z,
        &mut max_cell.z,
        Map::min_cell().z,
        Map::max_cell().z,
    ) {
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

/// returns if it moved min and max
fn move_inside_range(min: &mut i32, max: &mut i32, hard_min: i32, hard_max: i32) -> bool {
    if *min < hard_min {
        let diff = hard_min - *min;
        *min += diff;
        *max += diff;
        true
    } else if *max > hard_max {
        let diff = hard_max - *max;
        *min += diff;
        *max += diff;
        true
    } else {
        false
    }
}

/// returns if it moved min and max
fn move_inside_range_min_equals(
    min: &mut i32,
    max: &mut i32,
    hard_min: i32,
    hard_max: i32,
) -> bool {
    if *min <= hard_min {
        let diff = hard_min - *min;
        *min += diff;
        *max += diff;
        true
    } else if *max > hard_max {
        let diff = hard_max - *max;
        *min += diff;
        *max += diff;
        true
    } else {
        false
    }
}

fn highlight_cells_from_pixels(
    start: PixelPosition,
    end: PixelPosition,
    screen_width: f32,
    drawing_: &mut Drawing,
) -> (CellIndex, CellIndex) {
    let start_cell = clicked_cell(start, screen_width, drawing_);
    let end_cell = clicked_cell(end, screen_width, drawing_);
    higlight_cells(start_cell, end_cell, &mut drawing_.highlighted_cells);
    println!(
        "highlighted cells size: {}",
        drawing_.highlighted_cells.len()
    );
    (start_cell, end_cell)
}

fn higlight_cells(
    start_cell: CellIndex,
    end_cell: CellIndex,
    highlighted_cells: &mut HashSet<CellIndex>,
) {
    let cell_cube = CellCubeIterator::new_from_mixed(start_cell, end_cell);
    highlighted_cells.clear();
    for cell in cell_cube {
        highlighted_cells.insert(cell);
    }
}
