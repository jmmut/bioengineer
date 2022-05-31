pub mod input_macroquad;

pub trait InputSourceTrait {
    fn get_input() -> Input;
}

pub struct Input {
    pub quit: bool,
}
