use crate::input::CellSelectionType::{
    NoSelection, SelectionFinished, SelectionInProgress, SelectionStarted,
};

pub type PixelPosition = crate::Vec2;

pub trait InputSourceTrait {
    fn get_input(&mut self) -> Input;
}

pub struct Input {
    pub quit: bool,
    pub regenerate_map: bool,
    pub toggle_fluids: bool,
    pub single_fluid: bool,
    pub change_height_rel: i32,
    pub move_map_horizontally: PixelPosition,
    pub cell_selection: CellSelection,
    pub robot_movement: Option<PixelPosition>,
}

#[derive(Copy, Clone)]
pub struct CellSelection {
    pub state: CellSelectionType,
    pub selection: Option<PixelSelection>,
}

impl CellSelection {
    pub fn no_selection() -> Self {
        Self {
            state: NoSelection,
            selection: Option::None,
        }
    }
    pub fn started(selection: PixelSelection) -> Self {
        Self {
            state: SelectionStarted,
            selection: Option::Some(selection),
        }
    }
    pub fn in_progress(selection: PixelSelection) -> Self {
        Self {
            state: SelectionInProgress,
            selection: Option::Some(selection),
        }
    }
    pub fn finished(selection: PixelSelection) -> Self {
        Self {
            state: SelectionFinished,
            selection: Option::Some(selection),
        }
    }
}

#[derive(PartialEq, Copy, Clone)]
pub enum CellSelectionType {
    NoSelection,
    SelectionStarted,
    SelectionInProgress,
    SelectionFinished,
}

#[derive(Default, Debug, Copy, Clone)]
pub struct PixelSelection {
    pub start: PixelPosition,
    pub end: PixelPosition,
}
