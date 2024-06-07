pub mod change_height;
pub mod highlight_cells;
pub mod move_horizontally;

use crate::screen::drawing_state::highlight_cells::merge_consolidated_and_in_progress;
use crate::screen::gui::GuiActions;
use crate::screen::main_scene_input::{CellSelectionType, ZoomChange};
use crate::world::map::CellIndex;
use mq_basics::{IVec2, Vec2, Vec3};
use std::collections::HashSet;

pub type TilePosition = IVec2;
pub type SubTilePosition = Vec2;
pub type SubCellIndex = Vec3;

pub const DEFAULT_RENDER_DEPTH: i32 = 10;
pub const DEFAULT_RENDER_HALF_SIDE: i32 = 10;

#[derive(Clone)]
pub struct DrawingState {
    pub min_cell: CellIndex,
    pub max_cell: CellIndex,
    pub subtile_offset: SubTilePosition,
    pub subcell_diff: SubCellIndex,
    pub top_bar_showing: TopBarShowing,
    pub zoom: f32,
    highlighted_cells_in_progress: HashSet<CellIndex>,
    highlighted_cells_consolidated: HashSet<CellIndex>,
    highlighted_cells_in_progress_type: CellSelectionType,
    highlight_start_height: Option<i32>,
}

#[derive(PartialEq, Copy, Clone)]
pub enum TopBarShowing {
    Goals,
    Help,
    None,
}

impl DrawingState {
    pub fn new() -> Self {
        Self::new_centered(CellIndex::new(0, 1, 0))
    }
    pub fn new_centered(ship_pos: CellIndex) -> Self {
        let side = DEFAULT_RENDER_HALF_SIDE;
        DrawingState {
            min_cell: ship_pos + CellIndex::new(-side, -DEFAULT_RENDER_DEPTH, -side),
            max_cell: ship_pos + CellIndex::new(side - 1, 0, side - 1),
            subtile_offset: SubTilePosition::new(0.0, 0.0),
            subcell_diff: SubCellIndex::new(0.0, 0.0, 0.0),
            zoom: 1.0,
            top_bar_showing: TopBarShowing::None,
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

    pub fn set_highlighted_cells(&mut self, cells: HashSet<CellIndex>) {
        self.highlighted_cells_consolidated = cells;
        self.highlighted_cells_in_progress.clear();
        self.highlighted_cells_in_progress_type = CellSelectionType::Add;
    }

    pub fn apply_input(&mut self, gui_actions: &GuiActions) {
        self.maybe_change_height_rel(gui_actions.change_height_rel, gui_actions.go_to_robot);
        self.maybe_move_map_horizontally(
            gui_actions.move_map_horizontally_diff,
            gui_actions.go_to_robot,
        );
        self.maybe_select_cells_from_pixels(&gui_actions.cell_selection);
        self.update_zoom(gui_actions.zoom_change);
    }

    pub fn re_center(&mut self, cell_index: CellIndex) {
        self.change_height_to(cell_index.y);
        self.maybe_move_map_horizontally_to(cell_index.x, cell_index.z);
    }

    pub fn update_zoom(&mut self, zoom_change: ZoomChange) {
        match zoom_change {
            ZoomChange::ZoomIn => {
                if self.zoom >= 1.0 {
                    self.zoom += 0.25;
                } else {
                    self.zoom *= 2.0;
                }
            }
            ZoomChange::ZoomOut => {
                if self.zoom <= 1.0 {
                    self.zoom *= 0.5;
                } else {
                    self.zoom -= 0.25;
                }
            }
            ZoomChange::None => {}
        }
    }
}
