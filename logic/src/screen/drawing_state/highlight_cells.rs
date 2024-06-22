use crate::screen::drawing_state::DrawingState;
use crate::screen::main_scene_input::{CellSelection, CellSelectionState, CellSelectionType};
use crate::world::map::cell_envelope::{is_horizontally_inside, Envelope};
use crate::world::map::{CellCubeIterator, CellIndex};
use std::collections::HashSet;

impl DrawingState {
    pub fn maybe_select_cells_from_pixels(&mut self, cell_selection: &CellSelection) {
        cell_selection.selection.map(|selection| {
            self.maybe_select_cells(
                selection.start,
                selection.end,
                cell_selection.state,
                cell_selection.selection_type,
            );
        });
    }

    fn maybe_select_cells(
        &mut self,
        mut start: CellIndex,
        end: CellIndex,
        selection_state: CellSelectionState,
        selection_type: CellSelectionType,
    ) {
        if self.cell_index_set.highlighted_cells_in_progress_type != selection_type
            || selection_state == CellSelectionState::Started
        {
            self.finish_selection();
        }
        let shown_cube = Envelope {
            min_cell: self.min_cell,
            max_cell: self.max_cell,
        };
        if selection_state == CellSelectionState::Started {
            self.highlight_start_height = Some(self.max_cell.y);
        }
        if let Some(start_height) = self.highlight_start_height {
            start.y = start_height;
        } else {
            self.highlight_start_height = Some(start.y);
        }
        self.cell_index_set
            .highlight_cells(start, end, shown_cube, selection_type);
    }

    fn finish_selection(&mut self) {
        self.cell_index_set.finish_selection()
    }
}

fn highlight_cells(
    start: CellIndex,
    end: CellIndex,
    shown_cube: Envelope,
    _selection_type: CellSelectionType,
    highlighted_cells_in_progress: &mut HashSet<CellIndex>,
    _highlighted_cells_consolidated: &mut HashSet<CellIndex>,
) {
    let cell_cube = CellCubeIterator::new_from_mixed(start, end);
    highlighted_cells_in_progress.clear();
    for cell in cell_cube {
        if is_horizontally_inside(&cell, &shown_cube) {
            highlighted_cells_in_progress.insert(cell);
        }
    }
}

// #[derive(Clone)]
pub struct CellIndexSet {
    highlighted_cells_in_progress: HashSet<CellIndex>,
    highlighted_cells_consolidated: HashSet<CellIndex>,
    highlighted_cells_in_progress_type: CellSelectionType,
    merged_cached: HashSet<CellIndex>,
}

impl CellIndexSet {
    pub fn new() -> Self {
        Self {
            highlighted_cells_in_progress: HashSet::new(),
            highlighted_cells_consolidated: HashSet::new(),
            highlighted_cells_in_progress_type: CellSelectionType::Exclusive,
            merged_cached: HashSet::new(),
        }
    }
    pub fn highlight_cells(
        &mut self,
        start: CellIndex,
        end: CellIndex,
        shown_cube: Envelope,
        selection_type: CellSelectionType,
    ) {
        highlight_cells(
            start,
            end,
            shown_cube,
            selection_type,
            &mut self.highlighted_cells_in_progress,
            &mut self.highlighted_cells_consolidated,
        );
        self.highlighted_cells_in_progress_type = selection_type;
        self.merged_cached = self.merge_consolidated_and_in_progress();
    }

    fn merged(&self) -> &HashSet<CellIndex> {
        &self.merged_cached
    }
    pub fn contains(&self, cell_index: &CellIndex) -> bool {
        self.merged().contains(cell_index)
    }
    pub fn len(&self) -> usize {
        self.merged().len()
    }
    pub fn first(&self) -> Option<CellIndex> {
        self.merged().iter().next().cloned()
    }
    pub fn set_highlighted_cells(&mut self, cells: HashSet<CellIndex>) {
        self.highlighted_cells_consolidated = cells;
        self.highlighted_cells_in_progress.clear();
        self.highlighted_cells_in_progress_type = CellSelectionType::Add;
        self.merged_cached = self.merge_consolidated_and_in_progress();
    }
    fn finish_selection(&mut self) {
        self.highlighted_cells_consolidated = self.merge_consolidated_and_in_progress();
        self.highlighted_cells_in_progress.clear();
        self.merged_cached = self.merge_consolidated_and_in_progress();
    }
    pub fn highlighted_cells(&self) -> &HashSet<CellIndex> {
        self.merged()
    }
    pub fn merge_consolidated_and_in_progress(&self) -> HashSet<CellIndex> {
        merge_consolidated_and_in_progress(
            &self.highlighted_cells_consolidated,
            &self.highlighted_cells_in_progress,
            self.highlighted_cells_in_progress_type,
        )
    }
}

pub fn merge_consolidated_and_in_progress(
    consolidated: &HashSet<CellIndex>,
    in_progress: &HashSet<CellIndex>,
    selection_type: CellSelectionType,
) -> HashSet<CellIndex> {
    match selection_type {
        CellSelectionType::Exclusive => in_progress.clone(),
        CellSelectionType::Add => consolidated.union(&in_progress).cloned().collect(),
        CellSelectionType::Remove => consolidated.difference(&in_progress).cloned().collect(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::screen::main_scene_input::CellSelectionState::*;
    use crate::screen::main_scene_input::CellSelectionType::*;
    use crate::world::robots::up;

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
            Exclusive,
            &mut highlighted,
            &mut highlighted_consolidated,
        );
        assert_eq!(highlighted.len(), 1);
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

        drawing.maybe_select_cells(t.start, t.small_end, Finished, Exclusive);
        assert_eq!(drawing.highlighted_cells().len(), 4);
    }

    #[test]
    fn quick_selection_progress() {
        let mut drawing = DrawingState::new();
        let t = setup();

        drawing.maybe_select_cells(t.start, t.small_end, InProgress, Exclusive);
        assert_eq!(drawing.highlighted_cells().len(), 4);
    }

    #[test]
    fn new_selection() {
        let mut drawing = DrawingState::new();
        let t = setup();

        drawing.maybe_select_cells(t.start, t.small_end, Finished, Exclusive);
        assert_eq!(drawing.highlighted_cells().len(), 4);

        drawing.maybe_select_cells(t.small_end, t.big_end, Started, Exclusive);
        assert_eq!(
            *drawing.highlighted_cells_merged(),
            HashSet::from([t.small_end, t.big_end])
        );
    }

    #[test]
    fn selection_without_addition() {
        let mut drawing = DrawingState::new();
        let t = setup();

        drawing.maybe_select_cells(t.start, t.small_end, Started, Exclusive);
        assert_eq!(drawing.highlighted_cells().len(), 4);

        drawing.maybe_select_cells(t.start, t.big_end, InProgress, Exclusive);
        assert_eq!(drawing.highlighted_cells().len(), 6);

        drawing.maybe_select_cells(t.start, t.small_end, Started, Exclusive);
        assert_eq!(drawing.highlighted_cells().len(), 4);

        drawing.maybe_select_cells(t.start, t.big_end, Finished, Exclusive);
        assert_eq!(drawing.highlighted_cells().len(), 6);
    }

    #[test]
    fn selection_with_addition() {
        let mut drawing = DrawingState::new();
        let t = setup();

        drawing.maybe_select_cells(t.start, t.small_end, Finished, Exclusive);
        assert_eq!(drawing.highlighted_cells().len(), 4);

        drawing.maybe_select_cells(t.small_end, t.big_end, Started, Add);
        assert_eq!(drawing.highlighted_cells().len(), 5);

        drawing.maybe_select_cells(t.small_end, t.big_end + up(), InProgress, Add);
        assert_eq!(drawing.highlighted_cells().len(), 7);

        drawing.maybe_select_cells(t.small_end, t.big_end, InProgress, Add);
        assert_eq!(drawing.highlighted_cells().len(), 5);

        drawing.maybe_select_cells(t.small_end, t.big_end, Finished, Add);
        assert_eq!(drawing.highlighted_cells().len(), 5);
    }

    #[test]
    fn reduce_selection_with_addition() {
        let mut drawing = DrawingState::new();
        let t = setup();

        drawing.maybe_select_cells(t.small_end, t.big_end + up(), Started, Add);
        assert_eq!(drawing.highlighted_cells().len(), 4);

        drawing.maybe_select_cells(t.small_end, t.big_end, InProgress, Add);
        assert_eq!(drawing.highlighted_cells().len(), 2);
    }

    #[test]
    fn remove_selection() {
        let mut drawing = DrawingState::new();
        let t = setup();

        drawing.maybe_select_cells(t.start, t.big_end + up(), Finished, Add);
        assert_eq!(drawing.highlighted_cells().len(), 12);

        drawing.maybe_select_cells(t.start, t.small_end, Started, Remove);
        assert_eq!(drawing.highlighted_cells().len(), 8);

        drawing.maybe_select_cells(t.start, t.small_end, Finished, Remove);
        assert_eq!(drawing.highlighted_cells().len(), 8);
    }
}
