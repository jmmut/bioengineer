use crate::screen::drawer_trait::DrawerTrait;
use crate::screen::drawing_state::DrawingState;
use crate::screen::gui::format_units::{Grams, Watts};
use crate::screen::gui::panels::longest;
use crate::screen::gui::panels::top_bar::TOP_BAR_HEIGHT;
use crate::screen::gui::{GuiActions, FONT_SIZE};
use crate::screen::main_scene_input::CellSelection;
use crate::world::map::cell::is_networkable;
use crate::world::map::{is_liquid_or_air, is_walkable_horizontal, Cell, CellIndex, TileType};
use crate::world::networks::network::{POWER_PER_SOLAR_PANEL, STORAGE_PER_STORAGE_MACHINE};
use crate::world::networks::Networks;
use crate::world::World;

pub fn draw_cell_info(
    drawer: &mut dyn DrawerTrait,
    world: &World,
    drawing: &mut DrawingState,
    mut gui_actions: GuiActions,
) -> GuiActions {
    let highlighted_cells = drawing.highlighted_cells();
    if highlighted_cells.len() == 1 {
        let selected = highlighted_cells.iter().next().unwrap();
        let cell = world.map.get_cell(*selected);
        let cell_description = cell_to_str(cell, *selected, &world.networks);
        let panel_title = "Cell information".to_string();
        let longest_line = longest(cell_description.iter(), &panel_title);
        let max_line_width = drawer.ui_measure_text(longest_line.as_str(), FONT_SIZE).x;
        let panel_margin = 10.0;
        let big_margin_x = panel_margin + 1.0 * FONT_SIZE;
        let panel_width = max_line_width + 2.0 * big_margin_x;
        let line_height = FONT_SIZE * 1.5;
        let panel_height = (cell_description.len() + 2).max(5) as f32 * line_height;
        let interaction = drawer.ui_named_group(
            panel_title.as_str(),
            drawer.screen_width() - panel_width - panel_margin,
            panel_margin + TOP_BAR_HEIGHT,
            panel_width,
            panel_height,
            &mut |drawer| {
                for line in &cell_description {
                    drawer.ui_text(&line);
                }
            },
        );
        if interaction.is_hovered_or_clicked() {
            gui_actions.cell_selection = CellSelection::no_selection();
        }
    }
    gui_actions
}

fn cell_to_str(cell: &Cell, pos: CellIndex, networks: &Networks) -> Vec<String> {
    let tile = cell.tile_type;
    let basic_name = match tile {
        TileType::Unset => "Unset cell",
        TileType::WallRock => "Wall of rock",
        TileType::WallDirt => "Wall of dirt",
        TileType::FloorRock => "Floor of rock",
        TileType::FloorDirt => "Floor of Dirt",
        TileType::Stairs => "Stairs",
        TileType::Air => {
            if cell.pressure <= 0 {
                "Air"
            } else {
                "Water"
            }
        }
        TileType::Wire => "Wire",
        TileType::MachineAssembler => "Assembler machine",
        TileType::MachineAirCleaner => "Air cleaner machine",
        TileType::MachineDrill => "Drill machine",
        TileType::MachineSolarPanel => "Solar panel machine",
        TileType::MachineShip => "Spaceship",
        TileType::MachineStorage => "Storage machine",
        TileType::TreeHealthy => "Tree (Healthy)",
        TileType::TreeSparse => "Tree (Sparse)",
        TileType::TreeDying => "Tree (Dying)",
        TileType::TreeDead => "Tree (Dead)",
    };
    let mut description = vec![basic_name.to_string()];
    if tile == TileType::MachineShip {
        description.push("Can't be deconstructed".to_string());
    }
    if is_liquid_or_air(tile) || is_walkable_horizontal(tile) || cell.pressure > 0 {
        description.push(format!("  Liquid pressure: {} ", cell.pressure));
        if cell.pressure == 0 && tile != TileType::Air {
            // println!("wut");
        }
    }
    if is_networkable(tile) {
        description.push("  Effect on network:".to_string());
        let option = networks.get(pos);
        if let Some(_node) = option {
            // description.push(format!(
            //     "  pos: ({} {} {})",
            //     node.position.x, node.position.y, node.position.z
            // ));
            if cell.tile_type == TileType::MachineStorage {
                description.push(format!(
                    "    +{} storage capacity",
                    Grams::format(STORAGE_PER_STORAGE_MACHINE)
                ));
            } else if cell.tile_type == TileType::MachineSolarPanel {
                description.push(format!(
                    "    +{} power",
                    Watts::format(POWER_PER_SOLAR_PANEL)
                ));
            } else if cell.tile_type == TileType::MachineAirCleaner {
                description.push(format!(
                    "    -{} power",
                    Watts::format(POWER_PER_SOLAR_PANEL)
                ));
            } else if cell.tile_type == TileType::MachineShip {
                description.push("    Able to construct other machines".to_string());
            }
        }
    } else if !networks.is_adjacent_to_ship_network(pos) {
        description.push("  Networking: unreachable".to_string());
    }
    // TODO: print if a machine is working or not?
    // TODO: print contents of wires?
    description
}
