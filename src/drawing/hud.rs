use crate::gui::{GuiActions, BACKGROUND_UI_COLOR, FONT_SIZE, TEXT_COLOR};
use crate::input::{CellSelection, Input};
use crate::map::mechanics::allowed_transformations;
use crate::map::TileType;
use crate::Rect;
use crate::{DrawingTrait, GameState};

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
        TEXT_COLOR,
    );
}

pub fn draw_level(drawer: &impl DrawingTrait, min_y: i32, max_y: i32) {
    let text = format!("height: [{}, {}]", min_y, max_y);
    drawer.draw_text(
        text.as_str(),
        20.0,
        drawer.screen_height() - FONT_SIZE * 1.0,
        FONT_SIZE,
        TEXT_COLOR,
    );
}

pub fn show_available_actions(
    drawer: &impl DrawingTrait,
    game_state: &GameState,
    input: Input,
) -> GuiActions {
    let drawing_ = drawer.drawing();
    if drawing_.highlighted_cells.len() > 0 {
        let transformations = allowed_transformations(&drawing_.highlighted_cells, game_state);
        let line_height = FONT_SIZE * 1.5;
        let buttons_height = transformations.len() as f32 * line_height;
        let panel_height = buttons_height + 2.0 * line_height;
        let panel_margin = 10.0;
        let margin_x = panel_margin + FONT_SIZE;
        let big_margin_x = panel_margin + 2.0 * FONT_SIZE;
        let margin_y = panel_margin + line_height;
        let panel_title = "Available actions:";
        let mut max_button_width = drawer.measure_text(panel_title, FONT_SIZE).x;
        for transformation in &transformations {
            let text = to_action_str(transformation.new_tile_type);
            max_button_width = f32::max(max_button_width, drawer.measure_text(text, FONT_SIZE).x);
        }

        let panel = Rect::new(
            panel_margin,
            panel_margin,
            max_button_width + 2.0 * big_margin_x,
            panel_height,
        );
        drawer.draw_rectangle(panel.x, panel.y, panel.w, panel.h, BACKGROUND_UI_COLOR);
        drawer.draw_text(panel_title, margin_x, margin_y, FONT_SIZE, TEXT_COLOR);
        let mut i = 1.0;
        let mut transformation_clicked = Option::None;
        for transformation in transformations {
            let y = margin_y + i * line_height - FONT_SIZE / 2.0;
            let text = to_action_str(transformation.new_tile_type);
            if drawer.do_button(text, big_margin_x, y) {
                transformation_clicked = Option::Some(transformation);
            }
            i += 1.0;
        }
        if let Option::Some(selection) = input.cell_selection.selection.clone() {
            if panel.contains(selection.end) {
                return GuiActions {
                    input: Input {
                        cell_selection: CellSelection::no_selection(),
                        ..input
                    },
                    selected_cell_transformation: transformation_clicked,
                };
            }
        }
    }
    return GuiActions {
        input,
        selected_cell_transformation: Option::None,
    };
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
