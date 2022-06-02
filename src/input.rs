pub trait InputSourceTrait {
    fn get_input(&mut self) -> Input;
}

pub struct Input {
    pub quit: bool,
    pub change_height_rel: i32,
    pub move_map_horizontally: (f32, f32),
}
