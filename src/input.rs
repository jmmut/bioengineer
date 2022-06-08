use crate::input::CellSelectionType::{NoSelection, SelectionFinished, SelectionStarted};
use macroquad::input::TouchPhase::Started;

pub type PixelPosition = crate::Vec2;

pub trait InputSourceTrait {
    fn get_input(&mut self) -> Input;
}

pub struct Input {
    pub quit: bool,
    pub change_height_rel: i32,
    pub move_map_horizontally: PixelPosition,
    pub cell_selection: CellSelection,
}

pub struct CellSelection {
    pub state: CellSelectionType,
    pub selection: Option<Selection>,
}

impl CellSelection {
    pub fn no_selection() -> Self {
        Self {
            state: NoSelection,
            selection: Option::None,
        }
    }
    pub fn started(selection: Selection) -> Self {
        Self {
            state: SelectionStarted,
            selection: Option::Some(selection),
        }
    }
    pub fn finished(selection: Selection) -> Self {
        Self {
            state: SelectionFinished,
            selection: Option::Some(selection),
        }
    }
}

pub enum CellSelectionType {
    NoSelection,
    SelectionStarted,
    SelectionFinished,
}

#[derive(Default, Debug)]
pub struct Selection {
    pub start: PixelPosition,
    pub end: PixelPosition,
}
