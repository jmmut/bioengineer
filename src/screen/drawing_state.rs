pub mod change_height;
pub mod highlight_cells;
pub mod move_horizontally;

use crate::screen::gui::GuiActions;
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
            highlight_start_height: None,
        }
    }

    pub fn highlighted_cells(&self) -> HashSet<CellIndex> {
        let mut cells = self.highlighted_cells_consolidated.clone();
        cells.extend((&self.highlighted_cells_in_progress).iter());
        cells
    }

    pub fn apply_input(&mut self, unhandled: &GuiActions, screen_width: f32) {
        let input = &unhandled.input;
        self.maybe_change_height_rel(input.change_height_rel, unhandled.go_to_robot);
        self.maybe_move_map_horizontally(
            input.move_map_horizontally,
            unhandled.go_to_robot,
            screen_width,
        );
        self.maybe_select_cells_from_pixels(&input.cell_selection, screen_width);
    }
}
