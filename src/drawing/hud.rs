use crate::Color;
use crate::{DrawingTrait, GameState};

const FONT_SIZE: f32 = 20.0;
const BLACK: Color = Color::new(0.0, 0.0, 0.0, 1.0);


pub fn draw_fps(drawer : &impl DrawingTrait, game_state: &GameState) {
    let fps = 1.0 / (game_state.current_frame_ts - game_state.previous_frame_ts);
    // println!(
    //     "now - previous ts: {} - {}, fps: {}, frame: {}",
    //     game_state.current_frame_ts, game_state.previous_frame_ts, fps, game_state.frame_index
    // );
    let text = format!("{:.0}", fps);
    drawer.draw_text(
        text.as_str(),
        drawer.screen_width() - FONT_SIZE * 2.0,
        20.0,
        FONT_SIZE,
        BLACK,
    );
}

pub fn draw_level(drawer : &impl DrawingTrait, min_y: i32, max_y: i32) {
    let text = format!("height: [{}, {}]", min_y, max_y);
    drawer.draw_text(
        text.as_str(),
        20.0,
        drawer.screen_height() - FONT_SIZE * 1.0,
        FONT_SIZE,
        BLACK,
    );
}

