

pub type PixelPosition = crate::Vec2;

pub trait InputSourceTrait {
    fn get_input(&mut self) -> Input;
}

pub struct Input {
    pub quit: bool,
    pub change_height_rel: i32,
    pub move_map_horizontally: PixelPosition,
    pub start_selection: Option<PixelPosition>,
    pub end_selection: Option<PixelPosition>,
    pub mouse_position: PixelPosition,
}
