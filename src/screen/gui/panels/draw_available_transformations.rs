use crate::screen::drawer_trait::{DrawerTrait, Interaction};
use crate::screen::drawing_state::DrawingState;
use crate::screen::gui::gui_actions::GuiActions;
use crate::screen::gui::FONT_SIZE;
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
    let cell_selection = unhandled_input.cell_selection;
    let highlighted_cells = drawing.highlighted_cells();
    if highlighted_cells.len() > 0 {
        let mut transformations = allowed_transformations(&highlighted_cells, &world.map);
        transformations.sort_by(|t_1, t_2| {
            to_action_str(t_1.new_tile_type).cmp(to_action_str(t_2.new_tile_type))
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

        let panel = Rect::new(panel_margin, panel_margin, panel_width, panel_height);
        let mut hovered_opt = None;
        drawer.ui_named_group(panel_title, panel.x, panel.y, panel.w, panel.h, || {
            for transformation in transformations {
                let text = to_action_str(transformation.new_tile_type);
                match drawer.ui_button(text) {
                    Interaction::Clicked => {
                        let transformation_task = TransformationTask {
                            to_transform: highlighted_cells.clone(),
                            transformation: transformation.clone(),
                        };
                        transformation_clicked = Option::Some(transformation_task);
                    }
                    Interaction::Hovered => {
                        hovered_opt = Some(transformation.new_tile_type);
                    }
                    Interaction::None => {}
                }
            }
        });
        if let Some(hovered) = hovered_opt {
            if let Some(tooltip) = to_tooltip_str(hovered) {
                drawer.ui_named_group(
                    to_action_str(hovered),
                    panel.x + panel.w + panel_margin,
                    panel.y,
                    panel.w,
                    panel.h,
                    || {
                        for line in tooltip {
                            drawer.ui_text(line);
                        }
                    },
                );
            }
        }
        // if let Option::Some(selection) = unhandled_input.input.cell_selection.pixel_selection {
        //     if panel.contains(selection.end) {
        //         // TODO: if clicking a button near the bottom of the panel, it selects a cell out
        //         //       of screen. maybe solved.
        //         cell_selection = CellSelection::no_selection();
        //     }
        // }
    }
    GuiActions {
        cell_selection,
        selected_cell_transformation: transformation_clicked,
        ..unhandled_input
    }
}

pub fn to_action_str(tile: TileType) -> &'static str {
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
    }
}

fn to_tooltip_str(tile: TileType) -> Option<Vec<&'static str>> {
    match tile {
        TileType::Unset => {
            panic!()
        }
        TileType::WallRock => None,
        TileType::WallDirt => None,
        TileType::FloorRock => Some(vec!["- Makes a rock floor", "- Disassembles machines"]),
        TileType::FloorDirt => None,
        TileType::Stairs => Some(vec!["- Gives access to", "  underground levels"]),
        TileType::Air => None,
        TileType::Wire => Some(vec![
            "- Connects machines to",
            "  be part of the",
            "  same network",
        ]),
        TileType::MachineAssembler => None,
        TileType::MachineAirCleaner => Some(vec!["- Consumes 1KW"]),
        TileType::MachineDrill => None,
        TileType::MachineSolarPanel => Some(vec![
            "- Produces 1KW",
            "- Can not be built",
            "  underground",
        ]),
        TileType::MachineShip => None,
        TileType::DirtyWaterSurface => None,
        TileType::CleanWaterSurface => None,
        TileType::DirtyWaterWall => None,
        TileType::CleanWaterWall => None,
    }
}
