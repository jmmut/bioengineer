use macroquad::miniquad::date::now;

pub struct GameState {
    pub frame_index: i32,
    pub previous_frame_ts: f64,
}

impl GameState {
    pub fn new() -> GameState {
        GameState {
            frame_index: 0,
            previous_frame_ts: now(),
        }
    }
    pub fn advance_frame(&mut self) {
        self.frame_index = (self.frame_index + 1) % 1000;
        self.previous_frame_ts = now();
    }
}
