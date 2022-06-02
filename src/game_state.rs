use super::map::Map;
use crate::now;

pub struct GameState {
    pub frame_index: i32,
    pub previous_frame_ts: f64,
    pub current_frame_ts: f64,
    pub map: Map,
}

impl GameState {
    pub fn new() -> GameState {
        let mut map = Map::new();
        map.regenerate();
        GameState {
            frame_index: 0,
            previous_frame_ts: now() - 1.0,
            current_frame_ts: now(),
            map,
        }
    }
    pub fn advance_frame(&mut self) {
        self.frame_index = (self.frame_index + 1) % 1000;
        self.previous_frame_ts = self.current_frame_ts;
        self.current_frame_ts = now();
    }
}
