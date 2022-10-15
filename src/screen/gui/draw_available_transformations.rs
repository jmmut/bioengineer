use crate::screen::drawer_trait::DrawerTrait;
use crate::screen::drawing_state::DrawingState;
use crate::screen::gui::gui_actions::GuiActions;
use crate::screen::gui::{BACKGROUND_UI_COLOR, FONT_SIZE, TEXT_COLOR};
use crate::screen::input::{CellSelection, Input};
use crate::world::map::transform_cells::allowed_transformations;
use crate::world::map::TileType;
use crate::world::TransformationTask;
use crate::World;
use crate::{Rect, Vec2};
use macroquad::hash;
use macroquad::ui::root_ui;
use macroquad::ui::widgets::{Group, Window};

pub fn show_available_transformations(
    drawer: &impl DrawerTrait,
    world: &World,
    unhandled_input: GuiActions,
    drawing: &DrawingState,
) -> GuiActions {
    let mut transformation_clicked = Option::None;
    let mut cell_selection = unhandled_input.input.cell_selection;
    if drawing.highlighted_cells.len() > 0 {
        let transformations = allowed_transformations(&drawing.highlighted_cells, &world.map);
        let line_height = FONT_SIZE * 1.5;
        let buttons_height = transformations.len() as f32 * line_height;
        let panel_height = 10.0 * line_height;
        let panel_margin = 10.0;
        let big_margin_x = panel_margin + 2.0 * FONT_SIZE;
        let panel_title = "Available actions";
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
        Window::new(hash!(), panel.point(), panel.size())
            .titlebar(true)
            .label(panel_title)
            .movable(false)
            .ui(&mut root_ui(), |ui| {
                // ui.label(None, panel_title);
                for transformation in transformations {
                    let text = to_action_str(transformation.new_tile_type);
                    if ui.button(None, text) {
                        let transformation_task = TransformationTask {
                            to_transform: drawing.highlighted_cells.clone(),
                            transformation,
                        };
                        transformation_clicked = Option::Some(transformation_task);
                    }
                }
            });
        if let Option::Some(selection) = unhandled_input.input.cell_selection.selection {
            if panel.contains(selection.end) {
                // TODO: if clicking a button near the bottom of the panel, it selects a cell out
                //       of screen. maybe solved.
                cell_selection = CellSelection::no_selection();
            }
        }
    }
    GuiActions {
        input: Input {
            cell_selection,
            ..unhandled_input.input
        },
        selected_cell_transformation: transformation_clicked,
        robot_movement: Option::None,
        ..unhandled_input
    }
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
        TileType::MachineAirCleaner => "Build air cleaner",
        TileType::MachineDrill => "Build drill",
        TileType::MachineSolarPanel => "Build solar panel",
        TileType::MachineShip => "Build space ship",
        TileType::DirtyWaterSurface => "Dirty water surface",
        TileType::CleanWaterSurface => "Clean water surface",
        TileType::DirtyWaterWall => "Dirty water wall",
        TileType::CleanWaterWall => "Clean water wall",
        TileType::Robot => "Build robot",
        TileType::Movement => "Move robot",
    }
}
