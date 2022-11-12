use crate::screen::input::CellSelectionState::{Finished, InProgress, None, Started};

pub type PixelPosition = crate::Vec2;

pub trait InputSourceTrait {
    fn get_input(&mut self) -> Input;
}

pub struct Input {
    pub quit: bool,
    pub regenerate_map: bool,
    pub toggle_profiling: bool,
    pub toggle_fluids: bool,
    pub single_fluid: bool,
    pub change_height_rel: i32,
    pub move_map_horizontally: PixelPosition,
    pub cell_selection: CellSelection,
    pub robot_movement: Option<PixelPosition>,
    pub reset_quantities: bool,
}

#[derive(Copy, Clone)]
pub struct CellSelection {
    pub state: CellSelectionState,
    pub selection: Option<PixelSelection>,
    pub addition: CellSelectionType,
}

impl CellSelection {
    pub fn no_selection() -> Self {
        Self {
            state: None,
            selection: Option::None,
            addition: CellSelectionType::Exclusive,
        }
    }
    pub fn started(selection: PixelSelection, addition: CellSelectionType) -> Self {
        Self {
            state: Started,
            selection: Option::Some(selection),
            addition,
        }
    }
    pub fn in_progress(selection: PixelSelection, addition: CellSelectionType) -> Self {
        Self {
            state: InProgress,
            selection: Option::Some(selection),
            addition,
        }
    }
    pub fn finished(selection: PixelSelection, addition: CellSelectionType) -> Self {
        Self {
            state: Finished,
            selection: Option::Some(selection),
            addition,
        }
    }
}

#[derive(PartialEq, Copy, Clone)]
pub enum CellSelectionState {
    None,
    Started,
    InProgress,
    Finished,
}

#[derive(PartialEq, Copy, Clone)]
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
