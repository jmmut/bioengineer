pub mod change_height;
pub mod highlight_cells;
pub mod move_horizontally;

use crate::screen::drawing_state::highlight_cells::merge_consolidated_and_in_progress;
use crate::screen::gui::GuiActions;
use crate::screen::input::CellSelectionType;
use crate::world::map::CellIndex;
use crate::{IVec2, Vec2, Vec3};
use std::collections::HashSet;

pub type TilePosition = IVec2;
pub type SubTilePosition = Vec2;
pub type SubCellIndex = Vec3;

pub struct DrawingState {
    pub min_cell: CellIndex,
    pub max_cell: CellIndex,
    pub subtile_offset: SubTilePosition,
    pub subcell_diff: SubCellIndex,
    highlighted_cells_in_progress: HashSet<CellIndex>,
    highlighted_cells_consolidated: HashSet<CellIndex>,
    highlighted_cells_in_progress_type: CellSelectionType,
    highlight_start_height: Option<i32>,
}

impl DrawingState {
    pub fn new() -> Self {
        DrawingState {
            min_cell: CellIndex::new(-10, -1, -10),
            max_cell: CellIndex::new(9, 1, 9),
            subtile_offset: SubTilePosition::new(0.0, 0.0),
            subcell_diff: SubCellIndex::new(0.0, 0.0, 0.0),
            highlighted_cells_in_progress: HashSet::new(),
            highlighted_cells_consolidated: HashSet::new(),
            highlighted_cells_in_progress_type: CellSelectionType::Exclusive,
            highlight_start_height: None,
        }
    }

    pub fn highlighted_cells(&self) -> HashSet<CellIndex> {
        merge_consolidated_and_in_progress(
            &self.highlighted_cells_consolidated,
            &self.highlighted_cells_in_progress,
            self.highlighted_cells_in_progress_type,
        )
    }

    pub fn apply_input(&mut self, unhandled: &GuiActions, screen_width: f32) {
        self.maybe_change_height_rel(unhandled.change_height_rel, unhandled.go_to_robot);
        self.maybe_move_map_horizontally(
            unhandled.move_map_horizontally_diff,
            unhandled.go_to_robot,
            screen_width,
        );
        self.maybe_select_cells_from_pixels(&unhandled.cell_selection, screen_width);
    }
}
