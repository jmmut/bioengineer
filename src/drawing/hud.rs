use crate::map::mechanics::allowed_transformations;
use crate::map::TileType;
use crate::Color;
use crate::{DrawingTrait, GameState};

const FONT_SIZE: f32 = 20.0;
const BLACK: Color = Color::new(0.0, 0.0, 0.0, 1.0);
const BACKGROUND_UI_COLOR: Color = Color::new(64.0 / 255.0, 64.0 / 255.0, 80.0 / 255.0, 1.0);

pub fn draw_fps(drawer: &impl DrawingTrait, game_state: &GameState) {
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

pub fn draw_level(drawer: &impl DrawingTrait, min_y: i32, max_y: i32) {
    let text = format!("height: [{}, {}]", min_y, max_y);
    drawer.draw_text(
        text.as_str(),
        20.0,
        drawer.screen_height() - FONT_SIZE * 1.0,
        FONT_SIZE,
        BLACK,
    );
}

pub fn show_available_actions(drawer: &impl DrawingTrait, game_state: &GameState) {
    let drawing_ = drawer.drawing();
    if drawing_.highlighted_cells.len() > 0 {
        let transformations = allowed_transformations(&drawing_.highlighted_cells, game_state);
        let line_height = FONT_SIZE * 1.5;
        let buttons_height = transformations.len() as f32 * line_height;
        let panel_height = buttons_height + 2.0 * line_height;
        let panel_margin = 10.0;
        let margin_x = panel_margin + FONT_SIZE;
        let margin_y = panel_margin + line_height;
        drawer.draw_rectangle(
            panel_margin,
            panel_margin,
            200.0,
            panel_height,
            BACKGROUND_UI_COLOR,
        );
        drawer.draw_text("Available actions:", margin_x, margin_y, FONT_SIZE, BLACK);
        let mut i = 1.0;
        for transformation in transformations {
            drawer.draw_text(
                to_action_str(transformation.new_tile_type),
                margin_y,
                margin_y + i * line_height,
                FONT_SIZE,
                BLACK,
            );
            i += 1.0;
        }
    }
}

fn to_action_str(tile: TileType) -> &'static str {
    match tile {
        TileType::Unset => {
            panic!()
        }
        TileType::WallRock => "Build rock wall",
        TileType::WallDirt => "Build dirt wall",
        TileType::FloorRock => "Flatten",
        TileType::FloorDirt => "Flatten",
        TileType::Stairs => "Build stairs",
        TileType::Air => "Remove cell",
        TileType::MachineAssembler => "Build assembler",
        TileType::MachineDrill => "Build drill",
        TileType::MachineSolarPanel => "Build solar panel",
        TileType::MachineShip => "Build space ship",
        TileType::DirtyWaterSurface => "Dirty water surface",
        TileType::CleanWaterSurface => "Clean water surface",
        TileType::DirtyWaterWall => "Dirty water wall",
        TileType::CleanWaterWall => "Clean water wall",
        TileType::Robot => "Build robot",
    }
}
