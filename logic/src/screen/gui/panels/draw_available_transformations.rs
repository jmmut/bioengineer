use crate::screen::drawer_trait::{DrawerTrait, Interaction};
use crate::screen::drawing_state::DrawingState;
use crate::screen::gui::gui_actions::GuiActions;
use crate::screen::gui::panels::longest;
use crate::screen::gui::panels::top_bar::TOP_BAR_HEIGHT;
use crate::screen::gui::{FONT_SIZE, MARGIN};
use crate::screen::main_scene_input::CellSelection;
use crate::world::map::transform_cells::allowed_transformations;
use crate::world::map::TileType;
use crate::world::{TransformationTask, World};
use mq_basics::Rect;

pub fn show_available_transformations(
    drawer: &mut dyn DrawerTrait,
    world: &World,
    unhandled_input: GuiActions,
    drawing: &DrawingState,
) -> GuiActions {
    let mut transformation_clicked = Option::None;
    let mut cell_selection = unhandled_input.cell_selection;
    let highlighted_cells = drawing.highlighted_cells();
    if highlighted_cells.len() > 0 {
        let mut transformations =
            allowed_transformations(&highlighted_cells.highlighted_cells(), &world.map);
        transformations.sort_by(|t_1, t_2| {
            to_action_str(t_1.new_tile_type).cmp(to_action_str(t_2.new_tile_type))
        });
        let line_height = FONT_SIZE * 1.5;
        let panel_title = "Available actions";
        let mut max_button_width = drawer.ui_measure_text(panel_title, FONT_SIZE).x;
        let panel_margin = MARGIN;
        let big_margin_x = panel_margin + 2.0 * FONT_SIZE;
        let panel_height =
            3.0 * line_height + (1 + transformations.len()) as f32 * 1.2 * line_height;
        let panel_width = max_button_width + 2.0 * big_margin_x;
        for transformation in &transformations {
            let text = to_action_str(transformation.new_tile_type);
            max_button_width =
                f32::max(max_button_width, drawer.ui_measure_text(text, FONT_SIZE).x);
        }

        let panel = Rect::new(
            panel_margin,
            panel_margin + TOP_BAR_HEIGHT,
            panel_width,
            panel_height,
        );
        let mut hovered_opt = None;
        let transformations_panel = drawer.ui_named_group(
            panel_title,
            panel.x,
            panel.y,
            panel.w,
            panel.h,
            &mut |drawer| {
                drawer.ui_text(&format!("(on {} cells)", highlighted_cells.len()));
                for transformation in &transformations {
                    let text = to_action_str(transformation.new_tile_type);
                    match drawer.ui_button(text) {
                        Interaction::Clicked | Interaction::Pressing => {
                            let transformation_task = TransformationTask::new(
                                highlighted_cells.highlighted_cells().clone(),
                                transformation.clone(),
                            );
                            transformation_clicked = Option::Some(transformation_task);
                        }
                        Interaction::Hovered => {
                            hovered_opt = Some(transformation.new_tile_type);
                        }
                        Interaction::None => {}
                    }
                }
                if transformations.len() == 0 {
                    drawer.ui_text("No available actions");
                }
            },
        );
        if let Some(hovered) = hovered_opt {
            if let Some(tooltip) = to_tooltip_str(hovered) {
                let title = to_action_str(hovered);
                let longest_line = longest(tooltip.iter(), &title);
                let max_line_width = drawer.ui_measure_text(longest_line, FONT_SIZE).x;
                let pad_width = panel_margin + 1.0 * FONT_SIZE;
                let panel_width = max_line_width + 2.0 * pad_width;
                let line_height = FONT_SIZE * 1.5;
                let panel_height = (tooltip.len() + 2).max(10) as f32 * line_height;
                drawer.ui_named_group(
                    title,
                    panel.x + panel.w + panel_margin,
                    panel.y,
                    panel_width,
                    panel_height,
                    &mut |drawer| {
                        for line in &tooltip {
                            drawer.ui_text(line);
                        }
                    },
                );
            }
        }
        if transformations_panel.is_hovered_or_clicked() || transformation_clicked.is_some() {
            cell_selection = CellSelection::no_selection();
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
        TileType::MachineStorage => "Build storage",
        // TileType::DirtyWaterSurface => "Dirty water surface",
        // TileType::CleanWaterSurface => "Clean water surface",
        // TileType::DirtyWaterWall => "Dirty water wall",
        // TileType::CleanWaterWall => "Clean water wall",
        TileType::TreeHealthy => "Plant tree",
        TileType::TreeSparse => "Plant sparse tree",
        TileType::TreeDying => "Plant dying tree",
        TileType::TreeDead => "Kill tree",
    }
}

fn to_tooltip_str(tile: TileType) -> Option<Vec<&'static str>> {
    match tile {
        TileType::Unset => {
            panic!()
        }
        TileType::WallRock => None,
        TileType::WallDirt => None,
        TileType::FloorRock => Some(vec![
            "- Makes a rock floor",
            "",
            "- Disassembles machines",
            "",
            "- Uproots plants",
        ]),
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
            "",
            "- Can not be built",
            "  underground",
        ]),
        TileType::MachineShip => None,
        TileType::MachineStorage => Some(vec![
            "- Increases storage capacity",
            "  by 9.9 tonnes (9.9 Mg)",
            "",
            "- Increases available material",
            "  for construction by the",
            "  same amount",
        ]),
        // TileType::DirtyWaterSurface => None,
        // TileType::CleanWaterSurface => None,
        // TileType::DirtyWaterWall => None,
        // TileType::CleanWaterWall => None,
        TileType::TreeHealthy => Some(vec![
            "- Toxic air will kill",
            "  the tree.",
            // "",
            // "- Darkness will kill",
            // "  the tree",
        ]),
        TileType::TreeSparse => None,
        TileType::TreeDying => None,
        TileType::TreeDead => None,
    }
}
