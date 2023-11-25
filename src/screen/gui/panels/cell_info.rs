use crate::screen::drawer_trait::DrawerTrait;
use crate::screen::drawing_state::DrawingState;
use crate::screen::gui::panels::top_bar::TOP_BAR_HEIGHT;
use crate::screen::gui::{GuiActions, FONT_SIZE};
use crate::world::map::cell::is_networkable;
use crate::world::map::{is_liquid_or_air, Cell, CellIndex, TileType};
use crate::world::networks::Networks;
use crate::world::World;

pub fn draw_cell_info(
    drawer: &mut dyn DrawerTrait,
    world: &World,
    drawing: &mut DrawingState,
    gui_actions: GuiActions,
) -> GuiActions {
    let highlighted_cells = drawing.highlighted_cells();
    if highlighted_cells.len() == 1 {
        let selected = highlighted_cells.iter().next().unwrap();
        let panel_title = "Cell information";
        let max_button_width = drawer.ui_measure_text(panel_title, FONT_SIZE).x;
        let panel_margin = 10.0;
        let big_margin_x = panel_margin + 2.0 * FONT_SIZE;
        let panel_width = max_button_width + 2.0 * big_margin_x;
        let line_height = FONT_SIZE * 1.5;
        let panel_height = 10.0 * line_height;
        let cell = world.map.get_cell(*selected);
        let cell_description = cell_to_str(cell, *selected, &world.networks);
        let _interaction = drawer.ui_named_group(
            panel_title,
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
        TileType::Air => "Air",
        TileType::Wire => "Wire",
        TileType::MachineAssembler => "Assembler machine",
        TileType::MachineAirCleaner => "Air cleaner machine",
        TileType::MachineDrill => "Drill machine",
        TileType::MachineSolarPanel => "Solar panel machine",
        TileType::MachineShip => "Spaceship",
        TileType::DirtyWaterSurface => "Polluted water surface",
        TileType::CleanWaterSurface => "Clean water surface",
        TileType::DirtyWaterWall => "Polluted water",
        TileType::CleanWaterWall => "Clean water",
        TileType::TreeHealthy => "Tree (Healthy)",
        TileType::TreeSparse => "Tree (Sparse)",
        TileType::TreeDying => "Tree (Dying)",
        TileType::TreeDead => "Tree (Dead)",
    };
    let mut description = vec![basic_name.to_string()];
    if tile == TileType::MachineShip {
        description.push("Can't be deconstructed".to_string());
    }
    if is_liquid_or_air(tile) {
        description.push(format!("- Liquid pressure: {} ", cell.pressure));
        if cell.pressure == 0 && tile != TileType::Air {
            // println!("wut");
        }
    }
    if is_networkable(tile) {
        description.push("- Networking:".to_string());
        let option = networks.get(pos);
        if let Some(node) = option {
            description.push(format!(
                "  pos: ({} {} {})",
                node.position.x, node.position.y, node.position.z
            ));
        }
    } else if !networks.is_adjacent_to_ship_network(pos) {
        description.push("- Networking:".to_string());
        description.push("  Unreachable".to_string());
    }
    // TODO: print if a machine is working or not?
    // TODO: print contents of wires?
    description
}
