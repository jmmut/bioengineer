use crate::screen::drawing_state::DrawingState;
use crate::screen::gui::coords::cell_pixel::clicked_cell;
use crate::screen::input::{CellSelection, CellSelectionType};
use crate::world::map::cell_envelope::{is_horizontally_inside, Envelope};
use crate::world::map::{CellCubeIterator, CellIndex};
use std::collections::HashSet;

impl DrawingState {
    pub fn maybe_select_cells_from_pixels(
        &mut self,
        cell_selection: &CellSelection,
        screen_width: f32,
    ) {
        match cell_selection.selection {
            None => {}
            Some(selection) => {
                let start_cell = clicked_cell(selection.start, screen_width, self);
                let end_cell = clicked_cell(selection.end, screen_width, self);

                self.maybe_select_cells(
                    start_cell,
                    end_cell,
                    cell_selection.state,
                    cell_selection.addition,
                );
            }
        }
    }

    fn maybe_select_cells(
        &mut self,
        mut start: CellIndex,
        end: CellIndex,
        selection_type: CellSelectionType,
        addition: bool,
    ) {
        let shown_cube = Envelope {
            min_cell: self.min_cell,
            max_cell: self.max_cell,
        };
        if selection_type == CellSelectionType::Started {
            self.highlight_start_height = Some(self.max_cell.y);
        }
        if let Some(start_height) = self.highlight_start_height {
            start.y = start_height;
        } else {
            self.highlight_start_height = Some(start.y);
        }
        highlight_cells(
            start,
            end,
            shown_cube,
            addition,
            &mut self.highlighted_cells_in_progress,
            &mut self.highlighted_cells_consolidated,
        );
        if selection_type == CellSelectionType::Finished {
            self.highlighted_cells_consolidated
                .extend(self.highlighted_cells_in_progress.drain());
        }
    }
}

fn highlight_cells(
    start: CellIndex,
    end: CellIndex,
    shown_cube: Envelope,
    addition: bool,
    highlighted_cells_in_progress: &mut HashSet<CellIndex>,
    _highlighted_cells_consolidated: &mut HashSet<CellIndex>,
) {
    let cell_cube = CellCubeIterator::new_from_mixed(start, end);
    if !addition {
        highlighted_cells_in_progress.clear();
    }
    for cell in cell_cube {
        if is_horizontally_inside(&cell, &shown_cube) {
            highlighted_cells_in_progress.insert(cell);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::world::robots::up;
    use crate::screen::input::CellSelectionType::*;

    #[test]
    fn test_select_higher_cell() {
        let envelope = Envelope {
            min_cell: CellIndex::new(0, -10, 0),
            max_cell: CellIndex::new(0, -5, 0),
        };
        let mut highlighted = HashSet::new();
        let mut highlighted_consolidated = HashSet::new();
        highlight_cells(
            CellIndex::new(0, 0, 0),
            CellIndex::new(0, 0, 0),
            envelope,
            false,
            &mut highlighted,
            &mut highlighted_consolidated,
        );
        assert_eq!(highlighted.len(), 1);
    }

    #[test]
    fn test_reduce_selection() {
        let start = CellIndex::new(0, 0, 0);
        let big_end = CellIndex::new(1, 0, 2);
        let small_end = CellIndex::new(1, 0, 1);
        let mut highlighted = HashSet::new();
        let mut highlighted_consolidated = HashSet::new();
        highlight_cells(
            start,
            big_end,
            Envelope {
                min_cell: start,
                max_cell: big_end,
            },
            false,
            &mut highlighted,
            &mut highlighted_consolidated,
        );
        assert_eq!(highlighted.len(), 6);

        highlight_cells(
            start,
            big_end,
            Envelope {
                min_cell: start,
                max_cell: small_end,
            },
            false,
            &mut highlighted,
            &mut highlighted_consolidated,
        );
        assert_eq!(highlighted.len(), 4);

        highlight_cells(
            start + up(),
            small_end + up(),
            Envelope {
                min_cell: start,
                max_cell: small_end + up(),
            },
            true,
            &mut highlighted,
            &mut highlighted_consolidated,
        );
        assert_eq!(highlighted.len(), 8);
    }

    struct TestData {
        pub start: CellIndex,
        pub small_end: CellIndex,
        pub big_end: CellIndex,
    }

    fn setup() -> TestData {
        TestData {
            start: CellIndex::new(0, 1, 0),
            small_end: CellIndex::new(1, 1, 1),
            big_end: CellIndex::new(1, 1, 2),
        }
    }

    #[test]
    fn quick_selection() {
        let mut drawing = DrawingState::new();
        let t = setup();

        drawing.maybe_select_cells(t.start, t.small_end, Finished, false);
        assert_eq!(drawing.highlighted_cells_in_progress.len(), 0);
        assert_eq!(drawing.highlighted_cells_consolidated.len(), 4);
    }
    #[test]
    fn quick_selection_progress() {
        let mut drawing = DrawingState::new();
        let t = setup();

        drawing.maybe_select_cells(t.start, t.small_end, InProgress, false);
        assert_eq!(drawing.highlighted_cells_in_progress.len(), 4);
        assert_eq!(drawing.highlighted_cells_consolidated.len(), 0);
    }

    #[test]
    fn selection_without_addition() {
        let mut drawing = DrawingState::new();
        let t = setup();

        drawing.maybe_select_cells(t.start, t.small_end, Started, false);
        assert_eq!(drawing.highlighted_cells_in_progress.len(), 4);
        assert_eq!(drawing.highlighted_cells_consolidated.len(), 0);

        drawing.maybe_select_cells(t.start, t.big_end, InProgress, false);
        assert_eq!(drawing.highlighted_cells_in_progress.len(), 6);
        assert_eq!(drawing.highlighted_cells_consolidated.len(), 0);

        drawing.maybe_select_cells(t.start, t.small_end, Started, false);
        assert_eq!(drawing.highlighted_cells_in_progress.len(), 4);
        assert_eq!(drawing.highlighted_cells_consolidated.len(), 0);

        drawing.maybe_select_cells(t.start, t.big_end, Finished, false);
        assert_eq!(drawing.highlighted_cells_in_progress.len(), 0);
        assert_eq!(drawing.highlighted_cells_consolidated.len(), 6);
    }

    #[test]
    fn selection_with_addition() {
        let mut drawing = DrawingState::new();
        let t = setup();

        drawing.maybe_select_cells(t.start, t.small_end, Finished, false);
        assert_eq!(drawing.highlighted_cells_in_progress.len(), 0);
        assert_eq!(drawing.highlighted_cells_consolidated.len(), 4);

        drawing.maybe_select_cells(t.small_end, t.big_end, Started, true);
        assert_eq!(drawing.highlighted_cells_in_progress.len(), 2);
        assert_eq!(drawing.highlighted_cells_consolidated.len(), 4);

        drawing.maybe_select_cells(t.small_end, t.big_end + up(), InProgress, true);
        assert_eq!(drawing.highlighted_cells_in_progress.len(), 4);
        assert_eq!(drawing.highlighted_cells_consolidated.len(), 4);

        drawing.maybe_select_cells(t.small_end, t.big_end, Finished, true);
        assert_eq!(drawing.highlighted_cells_in_progress.len(), 0);
        assert_eq!(drawing.highlighted_cells_consolidated.len(), 7);

    }
}
