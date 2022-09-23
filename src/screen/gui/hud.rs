//! HUD (Heads Up Display) is (in this project) the part of the GUI that is not interactive, only
//! visual information is displayed.

use crate::screen::drawer_trait::DrawerTrait;
use crate::screen::gui::{FONT_SIZE, TEXT_COLOR, TEXT_COLOR_ALARM};
use crate::GameState;
use crate::world::game_state::get_goal_air_cleaned_str;

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
    let text = format!(
        "Production: Air cleaned: {}, goal: {}",
        game_state.networks.get_total_air_cleaned_str(),
        get_goal_air_cleaned_str(),
    );
    drawer.draw_text(
        text.as_str(),
        20.0,
        drawer.screen_height() - FONT_SIZE * (4.0 + network_count as f32),
        FONT_SIZE,
        TEXT_COLOR,
    );
    let text = format!("Number of networks: {}", network_count);
    drawer.draw_text(
        text.as_str(),
        20.0,
        drawer.screen_height() - FONT_SIZE * (3.0 + network_count as f32),
        FONT_SIZE,
        TEXT_COLOR,
    );
    for (network_id, network) in game_state.networks.iter().enumerate() {
        let text = format!(
            "  Network #{} - Power generated: {}. Power required: {}. Air cleaning speed: {}",
            network_id + 1,
            network.get_power_generated_str(),
            network.get_power_required_str(),
            network.get_air_cleaned_speed_str(),
        );
        let text_color = if network.is_power_satisfied() {
            TEXT_COLOR
        } else {
            TEXT_COLOR_ALARM
        };
        drawer.draw_text(
            text.as_str(),
            20.0,
            drawer.screen_height() - FONT_SIZE * (2.0 + (network_count - network_id) as f32),
            FONT_SIZE,
            text_color,
        );
    }
}
