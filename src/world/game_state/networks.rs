use crate::world::game_state::robots::CellIndexDiff;
use crate::world::map::cell::is_networkable;
use crate::world::map::{CellIndex, TileType};
use std::collections::{HashSet, VecDeque};
use std::slice::{Iter, IterMut};

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
    air_cleaned: f64,
}

pub struct Network {
    pub nodes: Vec<Node>,
}

pub struct Node {
    pub position: CellIndex,
    pub tile: TileType,
}

pub fn is_adjacent(a: CellIndex, b: CellIndex) -> bool {
    let diff: CellIndexDiff = a - b;
    adjacent_positions().contains(&diff)
}

impl Networks {
    pub fn new() -> Self {
        Networks {
            networks: Vec::new(),
            air_cleaned: 0.0,
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
        for (i, network) in &mut self.networks.iter_mut().enumerate() {
            match network.replace_if_present(cell_index, new_machine) {
                Replacement::SplitNetwork => {
                    self.split_network(i);
                    return true;
                }
                Replacement::RegularReplacement => {
                    if self.networks.get(i).unwrap().len() == 0 {
                        self.networks.remove(i);
                    }
                    return true;
                }
                Replacement::NoReplacement => {}
            }
        }
        return false;
    }

    pub fn get_adjacent_networks(&self, cell_index: CellIndex) -> Vec<usize> {
        let mut adjacents = Vec::new();
        for (i, network) in self.networks.iter().enumerate() {
            if network.is_adjacent(cell_index) {
                adjacents.push(i);
            }
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

    pub fn update(&mut self) {
        for network in self.networks.iter_mut() {
            let update = network.update();
            self.air_cleaned += update.air_cleaned;
        }
    }

    pub fn len(&self) -> usize {
        self.networks.len()
    }

    pub fn iter(&self) -> Iter<Network> {
        self.networks.iter()
    }

    pub fn iter_mut(&mut self) -> IterMut<Network> {
        self.networks.iter_mut()
    }

    pub fn get_total_air_cleaned(&self) -> f64 {
        self.air_cleaned
    }

    pub fn get_total_air_cleaned_str(&self) -> String {
        let air_cleaned = self.get_total_air_cleaned();
        format_unit(air_cleaned, "L")
    }

    pub fn reset(&mut self) {
        self.air_cleaned = 0.0;
    }

    fn split_network(&mut self, network_index: usize) {
        let mut new_networks = self
            .networks
            .get(network_index)
            .unwrap()
            .copy_into_networks();
        self.networks.remove(network_index);
        self.networks.append(&mut new_networks.networks);
    }
}

const POWER_PER_SOLAR_PANEL: f64 = 1000.0;
const POWER_CONSUMED_PER_MACHINE: f64 = POWER_PER_SOLAR_PANEL;
const AIR_CLEANED_PER_CLEANER: f64 = 1.0;

#[derive(PartialEq)]
enum Replacement {
    SplitNetwork,
    RegularReplacement,
    NoReplacement,
}

struct NetworkUpdate {
    air_cleaned: f64,
}

impl Network {
    pub fn new() -> Self {
        Network { nodes: Vec::new() }
    }
    pub fn update(&mut self) -> NetworkUpdate {
        let mut air_cleaners = 0;
        for node in &self.nodes {
            match node.tile {
                TileType::MachineAirCleaner => air_cleaners += 1,
                TileType::MachineAssembler => {}
                _ => {}
            }
        }
        NetworkUpdate {
            air_cleaned: if self.is_power_satisfied() {
                air_cleaners as f64 * AIR_CLEANED_PER_CLEANER
            } else {
                0.0
            },
        }
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
        let machines_count = self.count_tiles_of_type_in(&[
            TileType::MachineDrill,
            TileType::MachineAssembler,
            TileType::MachineAirCleaner,
        ]);
        let power = machines_count as f64 * POWER_PER_SOLAR_PANEL;
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

    fn get_air_cleaned_speed(&self) -> f64 {
        if self.is_power_satisfied() {
            let air_cleaners_count = self.count_tiles_of_type_in(&[TileType::MachineAirCleaner]);
            let liters = air_cleaners_count as f64 * AIR_CLEANED_PER_CLEANER;
            liters
        } else {
            0.0
        }
    }

    pub fn get_air_cleaned_speed_str(&self) -> String {
        let air_cleaned = self.get_air_cleaned_speed();
        format_unit(air_cleaned, "L/s")
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

    fn replace_if_present(&mut self, cell_index: CellIndex, new_machine: TileType) -> Replacement {
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
                if self.is_connected() {
                    Replacement::RegularReplacement
                } else {
                    Replacement::SplitNetwork
                }
            } else {
                self.nodes.get_mut(i).unwrap().tile = new_machine;
                Replacement::RegularReplacement
            }
        } else {
            Replacement::NoReplacement
        };
    }

    pub fn is_adjacent(&self, cell_index: CellIndex) -> bool {
        for node in &self.nodes {
            if is_adjacent(node.position, cell_index) {
                return true;
            }
        }
        return false;
    }

    pub fn is_connected(&self) -> bool {
        if self.nodes.is_empty() {
            return true;
        }
        let mut reachable = HashSet::new();
        let mut queue = VecDeque::new();
        let first_position = self.nodes.first().unwrap().position;
        queue.push_back(first_position);
        reachable.insert(first_position);
        while let Some(position) = queue.pop_front() {
            let max_neighbours = adjacent_positions().len();
            let mut neighbours_found = 0;
            for other_node in &self.nodes {
                if !reachable.contains(&other_node.position)
                    && is_adjacent(position, other_node.position)
                {
                    queue.push_back(other_node.position);
                    reachable.insert(other_node.position);
                    neighbours_found += 1;
                    if neighbours_found == max_neighbours {
                        break;
                    }
                }
            }
        }
        return reachable.len() == self.nodes.len();
    }

    pub fn add(&mut self, node: Node) {
        self.nodes.push(node);
    }

    pub fn copy_into_networks(&self) -> Networks {
        let mut new_networks = Networks::new();
        for node in &self.nodes {
            new_networks.add(node.position, node.tile);
        }
        new_networks
    }

    pub fn join(&mut self, other: Network) {
        for node in other.nodes {
            self.add(node);
        }
    }
}

fn adjacent_positions() -> [CellIndexDiff; 6] {
    [
        CellIndexDiff::new(1, 0, 0),
        CellIndexDiff::new(-1, 0, 0),
        CellIndexDiff::new(0, 1, 0),
        CellIndexDiff::new(0, -1, 0),
        CellIndexDiff::new(0, 0, 1),
        CellIndexDiff::new(0, 0, -1),
    ]
}

pub fn format_unit(quantity: f64, unit_name: &str) -> String {
    let unsigned_quantity = quantity.abs().floor();
    if unsigned_quantity < KILO {
        format!("{} {}", round_with_some_decimals(quantity), unit_name)
    } else if unsigned_quantity < MEGA {
        format!(
            "{} K{}",
            round_with_some_decimals(quantity / KILO),
            unit_name
        )
    } else if unsigned_quantity < GIGA {
        format!(
            "{} M{}",
            round_with_some_decimals(quantity / MEGA),
            unit_name
        )
    } else if unsigned_quantity < TERA {
        format!(
            "{} G{}",
            round_with_some_decimals(quantity / GIGA),
            unit_name
        )
    } else if unsigned_quantity < PETA {
        format!(
            "{} T{}",
            round_with_some_decimals(quantity / TERA),
            unit_name
        )
    } else if unsigned_quantity < EXA {
        format!(
            "{} P{}",
            round_with_some_decimals(quantity / PETA),
            unit_name
        )
    } else if unsigned_quantity < ZETTA {
        format!(
            "{} E{}",
            round_with_some_decimals(quantity / EXA),
            unit_name
        )
    } else if unsigned_quantity < YOTTA {
        format!(
            "{} Z{}",
            round_with_some_decimals(quantity / ZETTA),
            unit_name
        )
    } else {
        format!(
            "{} Y{}",
            round_with_some_decimals(quantity / YOTTA),
            unit_name
        )
    }
}

fn round_with_some_decimals(quantity: f64) -> f64 {
    let unsigned_quantity = quantity.abs();
    if unsigned_quantity >= 100.0 {
        quantity.floor()
    } else if unsigned_quantity >= 10.0 {
        (quantity * 10.0).floor() / 10.0
    } else {
        (quantity * 100.0).floor() / 100.0
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
    fn test_split_network() {
        let mut networks = Networks::new();
        networks.add(CellIndex::new(0, 0, 0), TileType::MachineAssembler);
        networks.add(CellIndex::new(0, 0, 1), TileType::MachineAssembler);
        networks.add(CellIndex::new(0, 0, 2), TileType::MachineAssembler);
        assert_eq!(networks.len(), 1);
        networks.add(CellIndex::new(0, 0, 1), TileType::FloorRock);
        assert_eq!(networks.len(), 2);
    }

    #[test]
    fn test_air_cleaned_is_kept_when_spliting_network() {
        let expected_air_cleaned = 1.0;
        let mut networks = Networks::new();
        networks.add(CellIndex::new(0, 0, 0), TileType::MachineSolarPanel);
        networks.add(CellIndex::new(0, 1, 0), TileType::MachineAirCleaner);
        networks.update();
        assert_eq!(networks.get_total_air_cleaned(), expected_air_cleaned);
        networks.add(CellIndex::new(0, 0, 2), TileType::MachineAssembler);
        assert_eq!(networks.get_total_air_cleaned(), expected_air_cleaned);
        networks.add(CellIndex::new(0, 0, 1), TileType::MachineAssembler);
        assert_eq!(networks.get_total_air_cleaned(), expected_air_cleaned);
        networks.add(CellIndex::new(0, 0, 1), TileType::FloorRock);
        assert_eq!(networks.get_total_air_cleaned(), expected_air_cleaned);
    }

    #[test]
    fn test_connected() {
        let mut network = Network::new();
        assert_eq!(network.is_connected(), true);
        network.add(Node {
            position: CellIndex::new(0, 0, 0),
            tile: TileType::Unset,
        });
        assert_eq!(network.is_connected(), true);
        network.add(Node {
            position: CellIndex::new(0, 0, 2),
            tile: TileType::Unset,
        });
        assert_eq!(network.is_connected(), false);
        network.add(Node {
            position: CellIndex::new(0, 0, 1),
            tile: TileType::Unset,
        });
        assert_eq!(network.is_connected(), true);
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
        assert_eq!(format_unit(0.5, "W"), "0.5 W");
        assert_eq!(format_unit(-0.5, "W"), "-0.5 W");
        assert_eq!(format_unit(10.0, "W"), "10 W");
        assert_eq!(format_unit(999.0, "W"), "999 W");
        assert_eq!(format_unit(1000.0, "W"), "1 KW");
        assert_eq!(format_unit(1110.0, "W"), "1.11 KW");
        assert_eq!(format_unit(10000.0, "W"), "10 KW");
        assert_eq!(format_unit(10100.0, "W"), "10.1 KW");
        assert_eq!(format_unit(100100.0, "W"), "100 KW");
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
