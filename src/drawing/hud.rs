use crate::drawing::assets::{PIXELS_PER_TILE_HEIGHT, PIXELS_PER_TILE_WIDTH};
use crate::gui::{GuiActions, BACKGROUND_UI_COLOR, FONT_SIZE, TEXT_COLOR};
use crate::input::{CellSelection, Input};
use crate::map::transform_cells::allowed_transformations;
use crate::map::TileType;
use crate::Rect;
use crate::{DrawingTrait, GameState};

pub fn draw_fps(drawer: &impl DrawingTrait, game_state: &GameState) {
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

pub fn draw_robot_queue(drawer: &impl DrawingTrait, game_state: &GameState) {
    let mut column = 1.0;
    let icon_width = PIXELS_PER_TILE_WIDTH as f32;
    let pixel_height = drawer.screen_height() - PIXELS_PER_TILE_HEIGHT as f32 * 1.0;
    drawer.draw_transparent_texture(
        TileType::Robot,
        drawer.screen_width() - column * icon_width,
        pixel_height,
        1.0,
    );
    for task in &game_state.task_queue {
        column += 1.0;
        drawer.draw_transparent_texture(
            task.transformation.new_tile_type,
            drawer.screen_width() - column * icon_width,
            pixel_height,
            1.0,
        );
    }
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
    let drawing_ = game_state.get_drawing();
    let mut transformation_clicked = Option::None;
    let mut cell_selection = input.cell_selection.clone();
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
                // TODO: if clicking a button near the bottom of the panel, it selects a cell out
                // of screen
                cell_selection = CellSelection::no_selection();
            }
        }
    }
    return GuiActions {
        input: Input {
            cell_selection,
            ..input
        },
        selected_cell_transformation: transformation_clicked,
        robot_movement: Option::None,
    };
}

fn to_action_str(tile: TileType) -> &'static str {
    match tile {
        TileType::Unset => {
            panic!()
        }
        TileType::WallRock => "Build rock wall",
        TileType::WallDirt => "Build dirt wall",
        TileType::FloorRock => "Flatten rock",
        TileType::FloorDirt => "Flatten dirt",
        TileType::Stairs => "Build stairs",
        TileType::Air => "Remove cell",
        TileType::Wire => "Build plumbing",
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
