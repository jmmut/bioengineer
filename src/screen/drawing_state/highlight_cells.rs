use crate::screen::gui::coords::cell_pixel::clicked_cell;
use crate::screen::drawing_state::DrawingState;
use crate::screen::input::{CellSelection, CellSelectionType, PixelPosition};
use crate::world::map::cell_envelope::{is_horizontally_inside, Envelope};
use crate::world::map::{CellCubeIterator, CellIndex};
use std::collections::HashSet;

impl DrawingState {
    pub fn maybe_select_cells(&mut self, cell_selection: &CellSelection, screen_width: f32) {
        if cell_selection.state == CellSelectionType::SelectionStarted {
            self.highlight_start_height = self.max_cell.y;
        }
        if let Option::Some(selection) = &cell_selection.selection {
            self.highlight_cells_from_pixels(
                selection.start,
                self.highlight_start_height,
                selection.end,
                screen_width,
            );
        }
    }

    fn highlight_cells_from_pixels(
        &mut self,
        start: PixelPosition,
        start_level: i32,
        end: PixelPosition,
        screen_width: f32,
    ) {
        let mut start_cell = clicked_cell(start, screen_width, self);
        start_cell.y = start_level;
        let end_cell = clicked_cell(end, screen_width, self);
        let shown_cube = Envelope {
            min_cell: self.min_cell,
            max_cell: self.max_cell,
        };
        // println!("start level: {}, end level: {}", start_cell.y, end_cell.y);
        highlight_cells(
            start_cell,
            end_cell,
            shown_cube,
            &mut self.highlighted_cells,
        );
        // (start_cell, end_cell)
    }
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
