
pub trait InputSourceTrait {
    fn get_input() -> Input;
}

pub struct Input {
    pub quit: bool,
    pub change_height_rel: i32,
}
