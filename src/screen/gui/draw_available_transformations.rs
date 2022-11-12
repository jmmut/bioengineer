use crate::screen::drawer_trait::DrawerTrait;
use crate::screen::drawing_state::DrawingState;
use crate::screen::gui::gui_actions::GuiActions;
use crate::screen::gui::FONT_SIZE;
use crate::screen::input::{CellSelection, Input};
use crate::world::map::transform_cells::allowed_transformations;
use crate::world::map::TileType;
use crate::world::TransformationTask;
use crate::Rect;
use crate::World;

pub fn show_available_transformations(
    drawer: &impl DrawerTrait,
    world: &World,
    unhandled_input: GuiActions,
    drawing: &DrawingState,
) -> GuiActions {
    let mut transformation_clicked = Option::None;
    let mut cell_selection = unhandled_input.input.cell_selection;
    let highlighted_cells = drawing.highlighted_cells();
    if highlighted_cells.len() > 0 {
        let mut transformations = allowed_transformations(&highlighted_cells, &world.map);
        transformations.sort_by(|t_1, t_2| {
            to_action_str(t_1.new_tile_type ).cmp(to_action_str(t_2.new_tile_type))
        });
        let line_height = FONT_SIZE * 1.5;
        let panel_title = "Available actions";
        let mut max_button_width = drawer.measure_text(panel_title, FONT_SIZE).x;
        let panel_margin = 10.0;
        let big_margin_x = panel_margin + 2.0 * FONT_SIZE;
        let panel_height = 10.0 * line_height;
        let panel_width = max_button_width + 2.0 * big_margin_x;
        for transformation in &transformations {
            let text = to_action_str(transformation.new_tile_type);
            max_button_width = f32::max(max_button_width, drawer.measure_text(text, FONT_SIZE).x);
        }

        let panel = Rect::new(panel_margin, panel_margin, panel_height, panel_height);
        drawer.ui_named_group(
            panel_title,
            panel_margin,
            panel_margin,
            panel_width,
            panel_height,
            || {
                for transformation in transformations {
                    let text = to_action_str(transformation.new_tile_type);
                    if drawer.ui_button(text) {
                        let transformation_task = TransformationTask {
                            to_transform: highlighted_cells.clone(),
                            transformation: transformation.clone(),
                        };
                        transformation_clicked = Option::Some(transformation_task);
                    }
                }
            },
        );
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
