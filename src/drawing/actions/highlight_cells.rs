use crate::drawing::coords::cell_pixel::clicked_cell;
use crate::drawing::Drawing;
use crate::input::PixelPosition;
use crate::world::map::cell_envelope::{is_horizontally_inside, Envelope};
use crate::world::map::{CellCubeIterator, CellIndex};
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
    let shown_cube = Envelope {
        min_cell: drawing_.min_cell,
        max_cell: drawing_.max_cell,
    };
    // println!("start level: {}, end level: {}", start_cell.y, end_cell.y);
    highlight_cells(
        start_cell,
        end_cell,
        shown_cube,
        &mut drawing_.highlighted_cells,
    );
    // (start_cell, end_cell)
}

fn highlight_cells(
    start_cell: CellIndex,
    end_cell: CellIndex,
    shown_cube: Envelope,
    highlighted_cells: &mut HashSet<CellIndex>,
) {
    let cell_cube = CellCubeIterator::new_from_mixed(start_cell, end_cell);
    highlighted_cells.clear();
    for cell in cell_cube {
        if is_horizontally_inside(&cell, &shown_cube) {
            highlighted_cells.insert(cell);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_select_higher_cell() {
        let envelope = Envelope {
            min_cell: CellIndex::new(0, -10, 0),
            max_cell: CellIndex::new(0, -5, 0),
        };
        let mut highlighted = HashSet::new();
        highlight_cells(
            CellIndex::new(0, 0, 0),
            CellIndex::new(0, 0, 0),
            envelope,
            &mut highlighted,
        );
        assert_eq!(highlighted.len(), 1);
    }
}
