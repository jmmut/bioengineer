use crate::game_state::robots::CellIndexDiff;
use crate::world::map::cell::is_networkable;
use crate::world::map::{CellIndex, TileType};
use std::slice::Iter;

const KILO: f64 = 1.0e3;
const MEGA: f64 = 1.0e6;
const GIGA: f64 = 1.0e9;
const TERA: f64 = 1.0e12;
const PETA: f64 = 1.0e15;
const EXA: f64 = 1.0e18;
const ZETTA: f64 = 1.0e21;
const YOTTA: f64 = 1.0e24;

pub struct Networks {
    networks: Vec<Network>,
}

pub struct Network {
    pub nodes: Vec<Node>,
}

pub struct Node {
    pub position: CellIndex,
    pub tile: TileType,
}

impl Networks {
    pub fn new() -> Self {
        Networks {
            networks: Vec::new(),
        }
    }

    pub fn add(&mut self, cell_index: CellIndex, new_machine: TileType) {
        if self.replace_if_present(cell_index, new_machine) {
            return;
        }
        if !is_networkable(new_machine) {
            return;
        }
        let node = Node {
            position: cell_index,
            tile: new_machine,
        };
        let adjacent_networks = self.get_adjacent_networks(cell_index);
        match adjacent_networks.split_first() {
            Option::Some((index_of_network_kept, indexes_of_networks_to_be_merged)) => {
                self.join_networks_and_add_node(
                    node,
                    *index_of_network_kept,
                    indexes_of_networks_to_be_merged,
                );
            }
            Option::None => {
                self.add_new_network_with_node(node);
            }
        }
    }

    fn replace_if_present(&mut self, cell_index: CellIndex, new_machine: TileType) -> bool {
        let mut index_network_replaced = Option::None;
        for (i, network) in &mut self.networks.iter_mut().enumerate() {
            if network.replace_if_present(cell_index, new_machine) {
                index_network_replaced = Option::Some(i);
                break;
            }
        }
        if let Option::Some(i) = index_network_replaced {
            if self.networks.get(i).unwrap().len() == 0 {
                self.networks.remove(i);
            }
        }
        return index_network_replaced.is_some();
    }

    pub fn get_adjacent_networks(&self, cell_index: CellIndex) -> Vec<usize> {
        let mut adjacents = Vec::new();
        let mut i = 0;
        for network in &self.networks {
            if network.is_adjacent(cell_index) {
                adjacents.push(i);
            }
            i += 1;
        }
        adjacents
    }

    fn join_networks_and_add_node(&mut self, node: Node, kept: usize, to_be_merged: &[usize]) {
        let mut networks_to_be_merged = Vec::new();
        for i in to_be_merged.iter().rev() {
            networks_to_be_merged.push(self.networks.remove(*i));
        }
        let network_kept = self.networks.get_mut(kept).unwrap();
        while let Option::Some(network_to_be_merged) = networks_to_be_merged.pop() {
            network_kept.join(network_to_be_merged);
        }
        network_kept.add(node)
    }

    fn add_new_network_with_node(&mut self, node: Node) {
        let mut network = Network::new();
        network.add(node);
        self.networks.push(network);
    }

    pub fn len(&self) -> usize {
        self.networks.len()
    }

    pub fn iter(&self) -> Iter<Network> {
        self.networks.iter()
    }
}

const POWER_PER_SOLAR_PANEL: f64 = 1000.0;

impl Network {
    pub fn new() -> Self {
        Network { nodes: Vec::new() }
    }

    pub fn len(&self) -> usize {
        self.nodes.len()
    }

    pub fn get_power_generated_str(&self) -> String {
        let power = self.get_power_generated();
        format_unit(power, "W")
    }

    fn get_power_generated(&self) -> f64 {
        let solar_panels_count = self.count_tiles_of_type_in(&[TileType::MachineSolarPanel]);
        let power = solar_panels_count as f64 * POWER_PER_SOLAR_PANEL;
        power
    }

    pub fn get_power_required_str(&self) -> String {
        let power = self.get_power_required();
        format_unit(power, "W")
    }

    fn get_power_required(&self) -> f64 {
        let solar_panels_count =
            self.count_tiles_of_type_in(&[TileType::MachineDrill, TileType::MachineAssembler]);
        let power = solar_panels_count as f64 * POWER_PER_SOLAR_PANEL;
        power
    }

    pub fn is_power_satisfied(&self) -> bool {
        self.get_power_generated() >= self.get_power_required()
    }

    fn count_tiles_of_type_in(&self, tiles: &[TileType]) -> i32 {
        let mut count = 0;
        for node in &self.nodes {
            if tiles.contains(&node.tile) {
                count += 1;
            }
        }
        count
    }

    #[allow(unused)]
    fn get(&mut self, cell_index: CellIndex) -> Option<&mut TileType> {
        for node in &mut self.nodes {
            if node.position == cell_index {
                return Option::Some(&mut node.tile);
            }
        }
        Option::None
    }

    fn replace_if_present(&mut self, cell_index: CellIndex, new_machine: TileType) -> bool {
        let mut index_to_change = Option::None;
        for (i, node) in self.nodes.iter().enumerate() {
            if node.position == cell_index {
                index_to_change = Option::Some(i);
                break;
            }
        }
        return if let Option::Some(i) = index_to_change {
            let should_remove = !is_networkable(new_machine);
            if should_remove {
                self.nodes.remove(i);
            } else {
                self.nodes.get_mut(i).unwrap().tile = new_machine;
            }
            true
        } else {
            false
        };
    }

    pub fn is_adjacent(&self, cell_index: CellIndex) -> bool {
        for node in &self.nodes {
            let diff: CellIndexDiff = node.position - cell_index;
            if adjacent_positions().contains(&diff) {
                return true;
            }
        }
        return false;
    }

    pub fn add(&mut self, node: Node) {
        self.nodes.push(node);
    }

    pub fn join(&mut self, other: Network) {
        for node in other.nodes {
            self.add(node);
        }
    }
}

fn adjacent_positions() -> Vec<CellIndexDiff> {
    vec![
        CellIndexDiff::new(1, 0, 0),
        CellIndexDiff::new(-1, 0, 0),
        CellIndexDiff::new(0, 1, 0),
        CellIndexDiff::new(0, -1, 0),
        CellIndexDiff::new(0, 0, 1),
        CellIndexDiff::new(0, 0, -1),
    ]
}

fn format_unit(quantity: f64, unit_name: &str) -> String {
    let unsigned_quantity = quantity.abs().floor();
    if unsigned_quantity < KILO {
        format!("{} {}", quantity.floor(), unit_name)
    } else if unsigned_quantity < MEGA {
        format!("{} K{}", (quantity / KILO).floor(), unit_name)
    } else if unsigned_quantity < GIGA {
        format!("{} M{}", (quantity / MEGA).floor(), unit_name)
    } else if unsigned_quantity < TERA {
        format!("{} G{}", (quantity / GIGA).floor(), unit_name)
    } else if unsigned_quantity < PETA {
        format!("{} T{}", (quantity / TERA).floor(), unit_name)
    } else if unsigned_quantity < EXA {
        format!("{} P{}", (quantity / PETA).floor(), unit_name)
    } else if unsigned_quantity < ZETTA {
        format!("{} E{}", (quantity / EXA).floor(), unit_name)
    } else if unsigned_quantity < YOTTA {
        format!("{} Z{}", (quantity / ZETTA).floor(), unit_name)
    } else {
        format!("{} Y{}", (quantity / YOTTA).floor(), unit_name)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::world::map::{CellIndex, TileType};

    #[test]
    fn test_join_networks() {
        let mut networks = Networks::new();
        networks.add(CellIndex::new(0, 0, 0), TileType::MachineAssembler);
        networks.add(CellIndex::new(0, 0, 2), TileType::MachineAssembler);
        assert_eq!(networks.len(), 2);
        networks.add(CellIndex::new(0, 0, 1), TileType::MachineAssembler);
        assert_eq!(networks.len(), 1);
        assert_eq!(networks.networks.get(0).unwrap().nodes.len(), 3);
    }

    #[test]
    fn test_append_to_network() {
        let mut networks = Networks::new();
        networks.add(CellIndex::new(0, 0, 0), TileType::MachineAssembler);
        networks.add(CellIndex::new(0, 0, 1), TileType::MachineAssembler);
        assert_eq!(networks.len(), 1);
    }

    #[test]
    fn test_replace_machine() {
        let mut networks = Networks::new();
        networks.add(CellIndex::new(0, 0, 0), TileType::MachineAssembler);
        networks.add(CellIndex::new(0, 0, 0), TileType::MachineDrill);
        assert_eq!(networks.len(), 1);
        assert_eq!(
            networks
                .networks
                .get(0)
                .unwrap()
                .nodes
                .first()
                .unwrap()
                .tile,
            TileType::MachineDrill
        );
    }

    #[test]
    fn test_destroy_machine() {
        let mut networks = Networks::new();
        networks.add(CellIndex::new(0, 0, 0), TileType::MachineAssembler);
        networks.add(CellIndex::new(0, 0, 1), TileType::MachineAssembler);
        networks.replace_if_present(CellIndex::new(0, 0, 1), TileType::FloorRock);
        assert_eq!(networks.len(), 1);
        assert_eq!(networks.networks.get(0).unwrap().nodes.len(), 1);
        networks.replace_if_present(CellIndex::new(0, 0, 0), TileType::FloorRock);
        assert_eq!(networks.len(), 0);
    }

    #[test]
    fn test_is_adjacent() {
        let mut network = Network::new();
        let adjacent = CellIndex::new(0, 0, 0);
        let position = CellIndex::new(0, 0, 1);
        assert_eq!(network.is_adjacent(adjacent), false);
        network.add(Node {
            tile: TileType::MachineDrill,
            position,
        });
        assert_eq!(network.is_adjacent(adjacent), true);
    }

    #[test]
    fn test_format_units() {
        assert_eq!(format_unit(1000.0, " paperclips"), "1 K paperclips");
        assert_eq!(format_unit(0.0, "W"), "0 W");
        assert_eq!(format_unit(0.5, "W"), "0 W");
        assert_eq!(format_unit(-0.5, "W"), "-1 W");
        assert_eq!(format_unit(999.0, "W"), "999 W");
        assert_eq!(format_unit(1000.0, "W"), "1 KW");
        assert_eq!(format_unit(999999.0, "W"), "999 KW");
        assert_eq!(format_unit(1000000.0, "W"), "1 MW");
        assert_eq!(format_unit(999999999.0, "W"), "999 MW");
        assert_eq!(format_unit(1000000000.0, "W"), "1 GW");
        assert_eq!(format_unit(999999999999.0, "W"), "999 GW");
        assert_eq!(format_unit(1000000000000.0, "W"), "1 TW");
        assert_eq!(format_unit(999999999999999.0, "W"), "999 TW");
        assert_eq!(format_unit(1000000000000000.0, "W"), "1 PW");
        assert_eq!(format_unit(999999999999999935.0, "W"), "999 PW");
        assert_eq!(format_unit(1000000000000000000.0, "W"), "1 EW");
        assert_eq!(format_unit(999999999999999934000.0, "W"), "999 EW");
        assert_eq!(format_unit(1000000000000000000000.0, "W"), "1 ZW");
        assert_eq!(format_unit(999999999999999916000000.0, "W"), "999 ZW");
        assert_eq!(format_unit(1000000000000000000000000.0, "W"), "1 YW");
        assert_eq!(format_unit(1000000000000000000000000000.0, "W"), "1000 YW");
    }
}
