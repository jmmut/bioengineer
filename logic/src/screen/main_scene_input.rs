use crate::world::map::CellIndex;
use juquad::PixelPosition;

#[derive(Copy, Clone)]
pub struct Input {
    pub quit: bool,
    pub regenerate_map: bool,
    pub reload_ui_skin: bool,
    pub toggle_profiling: bool,
    pub toggle_fluids: bool,
    pub single_fluid: bool,
    pub change_height_rel: i32,
    pub move_map_horizontally: PixelPosition,
    pub cell_selection: PixelCellSelection,
    pub robot_movement: Option<PixelPosition>,
    pub reset_quantities: bool,
    pub zoom_change: ZoomChange,
    pub go_to_ship: bool,
}

#[derive(Copy, Clone)]
pub enum ZoomChange {
    ZoomIn,
    ZoomOut,
    None,
}

#[derive(Copy, Clone)]
pub struct PixelCellSelection {
    pub state: CellSelectionState,
    pub pixel_selection: Option<PixelSelection>,
    pub selection_type: CellSelectionType,
}

#[derive(Copy, Clone)]
pub struct CellSelection {
    pub state: CellSelectionState,
    pub selection: Option<CellIndexSelection>,
    pub selection_type: CellSelectionType,
}

#[derive(PartialEq, Copy, Clone, Debug)]
pub enum CellSelectionState {
    None,
    Started,
    InProgress,
    Finished,
}

#[derive(PartialEq, Copy, Clone, Debug)]
pub enum CellSelectionType {
    Exclusive,
    Add,
    Remove,
}

#[derive(Default, Debug, Copy, Clone)]
pub struct PixelSelection {
    pub start: PixelPosition,
    pub end: PixelPosition,
}

#[derive(Default, Debug, Copy, Clone)]
pub struct CellIndexSelection {
    pub start: CellIndex,
    pub end: CellIndex,
}

impl PixelCellSelection {
    pub fn no_selection() -> Self {
        Self {
            state: CellSelectionState::None,
            pixel_selection: Option::None,
            selection_type: CellSelectionType::Exclusive,
        }
    }
    pub fn started(selection: PixelSelection, addition: CellSelectionType) -> Self {
        Self {
            state: CellSelectionState::Started,
            pixel_selection: Option::Some(selection),
            selection_type: addition,
        }
    }
    pub fn in_progress(selection: PixelSelection, addition: CellSelectionType) -> Self {
        Self {
            state: CellSelectionState::InProgress,
            pixel_selection: Option::Some(selection),
            selection_type: addition,
        }
    }
    pub fn finished(selection: PixelSelection, addition: CellSelectionType) -> Self {
        Self {
            state: CellSelectionState::Finished,
            pixel_selection: Option::Some(selection),
            selection_type: addition,
        }
    }
}

impl CellSelection {
    pub fn no_selection() -> Self {
        Self {
            state: CellSelectionState::None,
            selection: Option::None,
            selection_type: CellSelectionType::Exclusive,
        }
    }
    pub fn is_something_being_selected(&self) -> bool {
        self.state == CellSelectionState::Started || self.state == CellSelectionState::InProgress
    }
}
