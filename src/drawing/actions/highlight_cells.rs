use crate::drawing::coords::cell_pixel::clicked_cell;
use crate::drawing::Drawing;
use crate::input::PixelPosition;
use crate::map::{CellCubeIterator, CellIndex};
use std::collections::HashSet;

pub fn highlight_cells_from_pixels(
    start: PixelPosition,
    start_level: i32,
    end: PixelPosition,
    screen_width: f32,
    drawing_: &mut Drawing,
) {
    let mut start_cell = clicked_cell(start, screen_width, drawing_);
    start_cell.y = start_level;
    let end_cell = clicked_cell(end, screen_width, drawing_);
    // println!("start level: {}, end level: {}", start_cell.y, end_cell.y);
    higlight_cells(start_cell, end_cell, &mut drawing_.highlighted_cells);
    // (start_cell, end_cell)
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