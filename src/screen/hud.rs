use crate::GameState;
use crate::screen::drawer::DrawerTrait;
use crate::screen::gui::{FONT_SIZE, TEXT_COLOR, TEXT_COLOR_ALARM};

pub const FULL_OPAQUE: f32 = 1.0;

pub fn draw_fps(drawer: &impl DrawerTrait, game_state: &GameState) {
    let fps = 1.0 / (game_state.current_frame_ts - game_state.previous_frame_ts);
    if game_state.profile {
        println!(
            "frame: {} - frame time: {:.3} ms, fps: {:.3}, previous ts: {} - {}",
            game_state.frame_index,
            (game_state.current_frame_ts - game_state.previous_frame_ts) * 1000.0,
            fps,
            game_state.current_frame_ts,
            game_state.previous_frame_ts
        );
    }
    let text = format!("{:.0}", fps);
    drawer.draw_text(
        text.as_str(),
        drawer.screen_width() - FONT_SIZE * 2.0,
        20.0,
        FONT_SIZE,
        TEXT_COLOR,
    );
}

pub fn draw_level(drawer: &impl DrawerTrait, min_y: i32, max_y: i32) {
    let text = format!("height: [{}, {}]", min_y, max_y);
    drawer.draw_text(
        text.as_str(),
        20.0,
        drawer.screen_height() - FONT_SIZE * 1.0,
        FONT_SIZE,
        TEXT_COLOR,
    );
}

pub fn draw_networks(drawer: &impl DrawerTrait, game_state: &GameState) {
    let network_count = game_state.networks.len();
    let text = format!("Number of networks: {}", network_count);
    drawer.draw_text(
        text.as_str(),
        20.0,
        drawer.screen_height() - FONT_SIZE * (3.0 + network_count as f32),
        FONT_SIZE,
        TEXT_COLOR,
    );
    for (i, network) in game_state.networks.iter().enumerate() {
        let text = format!(
            "  Network #{} - Power generated: {}. Power required: {}.",
            i,
            network.get_power_generated_str(),
            network.get_power_required_str(),
        );
        let text_color = if network.is_power_satisfied() {
            TEXT_COLOR
        } else {
            TEXT_COLOR_ALARM
        };
        drawer.draw_text(
            text.as_str(),
            20.0,
            drawer.screen_height() - FONT_SIZE * (2.0 + (network_count - i) as f32),
            FONT_SIZE,
            text_color,
        );
    }
}
