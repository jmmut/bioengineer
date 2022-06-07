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

pub enum CellSelection {
    NoSelection,
    SelectionStarted(Selection),
    SelectionFinished(Selection),
}

#[derive(Default, Debug)]
pub struct Selection {
    pub start: PixelPosition,
    pub end: PixelPosition,
}
