pub mod network;

use crate::screen::gui::format_units::format_unit;
use crate::world::map::cell::is_networkable;
use crate::world::map::{CellIndex, TileType};
use crate::world::networks::network::{neighbours, Network, Node, Replacement};
use std::slice::Iter;

pub struct Networks {
    ship_position: CellIndex,
    ship_network: Network,
    unconnected_networks: Vec<Network>,
    air_cleaned: f64,
}

impl Networks {
    pub fn new(ship_position: CellIndex) -> Self {
        let mut network = Network::new();
        network.add(Node {
            position: ship_position,
            tile: TileType::MachineShip,
        });
        Networks {
            ship_position,
            ship_network: network,
            unconnected_networks: Vec::new(),
            air_cleaned: 0.0,
        }
    }

    pub fn new_default() -> Self {
        Self::new(CellIndex::default())
    }

    pub fn add(&mut self, cell_index: CellIndex, new_machine: TileType) -> bool {
        if self.replace_if_present(cell_index, new_machine) {
            return true;
        }
        if !is_networkable(new_machine) {
            return false;
        }
        let node = Node {
            position: cell_index,
            tile: new_machine,
        };
        if self.ship_network.is_adjacent(cell_index) {
            self.ship_network.add(node);
            let adjacent_networks = self.get_adjacent_networks(cell_index);
            for i_network in adjacent_networks.iter().rev() {
                let joining_network = self.unconnected_networks.remove(*i_network);
                for unconnected_nodes in joining_network.nodes {
                    self.ship_network.add(unconnected_nodes);
                }
            }
            return true
        }
        // not connected to ship_network
        let adjacent_networks = self.get_adjacent_networks(cell_index);
        if adjacent_networks.len() > 0 {
            self.join_networks_and_add_node(node, adjacent_networks[0], &adjacent_networks);
        } else {
            self.add_new_network_with_node(node);
        };
        return true;
    }

    fn replace_if_present(&mut self, cell_index: CellIndex, new_machine: TileType) -> bool {
        for (i, network) in &mut self.unconnected_networks.iter_mut().enumerate() {
            match network.replace_if_present(cell_index, new_machine) {
                Replacement::SplitNetwork => {
                    self.split_network(i);
                    return true;
                }
                Replacement::Regular => {
                    if self.unconnected_networks.get(i).unwrap().len() == 0 {
                        self.unconnected_networks.remove(i);
                    }
                    return true;
                }
                Replacement::Forbidden => {return false;}
                Replacement::None => {}
            }
        }
        return false;
    }

    fn get_adjacent_networks(&self, cell_index: CellIndex) -> Vec<usize> {
        let mut adjacents = Vec::new();
        for (i, network) in self.unconnected_networks.iter().enumerate() {
            if network.is_adjacent(cell_index) {
                adjacents.push(i);
            }
        }
        adjacents
    }
    pub fn is_adjacent_to_unconnected_networks(&self, cell_index: CellIndex) -> bool {
        self.get_adjacent_networks(cell_index).len() > 0
    }
    pub fn is_adjacent_to_ship_network(&self, cell_index: CellIndex) -> bool {
        self.ship_network.is_adjacent(cell_index)
    }

    fn join_networks_and_add_node(&mut self, node: Node, kept: usize, to_be_merged: &[usize]) {
        let mut networks_to_be_merged = Vec::new();
        for i in to_be_merged.iter().rev() {
            networks_to_be_merged.push(self.unconnected_networks.remove(*i));
        }
        let network_kept = self.unconnected_networks.get_mut(kept).unwrap();
        while let Option::Some(network_to_be_merged) = networks_to_be_merged.pop() {
            network_kept.join(network_to_be_merged);
        }
        network_kept.add(node)
    }

    fn add_new_network_with_node(&mut self, node: Node) {
        let mut network = Network::new();
        network.add(node);
        self.unconnected_networks.push(network);
    }

    pub fn update(&mut self) {
        for network in self.unconnected_networks.iter_mut() {
            let update = network.update();
            self.air_cleaned += update.air_cleaned;
        }
    }

    pub fn len(&self) -> usize {
        self.unconnected_networks.len()
    }

    pub fn get_non_ship_machine_count(&self) -> i32 {
        let mut machines = 0;
        for network in &self.unconnected_networks {
            machines += network.len() as i32;
        }
        machines -1
    }

    pub fn iter(&self) -> Iter<Network> {
        self.unconnected_networks.iter()
    }

    pub fn get_total_air_cleaned(&self) -> f64 {
        self.air_cleaned
    }

    pub fn get_total_air_cleaned_str(&self) -> String {
        let air_cleaned = self.get_total_air_cleaned();
        format_unit(air_cleaned, "L")
    }

    pub fn reset_production(&mut self) {
        self.air_cleaned = 0.0;
    }

    #[cfg(test)]
    pub fn set_production(&mut self, air_cleaned: f64) {
        self.air_cleaned = air_cleaned;
    }

    fn split_network(&mut self, network_index: usize) {
        let network_to_split = self.unconnected_networks.swap_remove(network_index);
        for node in network_to_split.nodes {
            self.add(node.position, node.tile);
        }
    }

    pub fn clear(&mut self) {
        self.unconnected_networks.clear();
        self.reset_production();
    }

    pub fn get(&self, position: CellIndex) -> Option<Node> {
        for network in &self.unconnected_networks {
            let node_opt = network.get_node(position);
            if node_opt.is_some() {
                return node_opt;
            }
        }
        None
    }
    pub fn is_in_ship_network(&self, position: CellIndex) -> bool {
        let node_opt = self.ship_network.get_node(position);
        return node_opt.is_some();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::world::map::{CellIndex, TileType};

    #[test]
    fn test_join_networks() {
        let mut networks = Networks::new_default();
        networks.add(CellIndex::new(0, 0, 0), TileType::MachineAssembler);
        networks.add(CellIndex::new(0, 0, 2), TileType::MachineAssembler);
        assert_eq!(networks.len(), 2);
        networks.add(CellIndex::new(0, 0, 1), TileType::MachineAssembler);
        assert_eq!(networks.len(), 1);
        assert_eq!(networks.unconnected_networks.get(0).unwrap().nodes.len(), 3);
    }

    #[test]
    fn test_append_to_network() {
        let mut networks = Networks::new_default();
        networks.add(CellIndex::new(0, 0, 0), TileType::MachineAssembler);
        networks.add(CellIndex::new(0, 0, 1), TileType::MachineAssembler);
        assert_eq!(networks.len(), 1);
    }

    #[test]
    fn test_replace_machine() {
        let mut networks = Networks::new_default();
        networks.add(CellIndex::new(0, 0, 0), TileType::MachineAssembler);
        networks.add(CellIndex::new(0, 0, 0), TileType::MachineDrill);
        assert_eq!(networks.len(), 1);
        assert_eq!(
            networks
                .unconnected_networks
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
        let mut networks = Networks::new_default();
        networks.add(CellIndex::new(0, 0, 0), TileType::MachineAssembler);
        networks.add(CellIndex::new(0, 0, 1), TileType::MachineAssembler);
        networks.replace_if_present(CellIndex::new(0, 0, 1), TileType::FloorRock);
        assert_eq!(networks.len(), 1);
        assert_eq!(networks.unconnected_networks.get(0).unwrap().nodes.len(), 1);
        networks.replace_if_present(CellIndex::new(0, 0, 0), TileType::FloorRock);
        assert_eq!(networks.len(), 0);
    }

    #[test]
    fn test_split_network() {
        let mut networks = Networks::new_default();
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
        let mut networks = Networks::new_default();
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

    fn new_node(position: CellIndex, tile: TileType) -> Node {
        Node {position, tile, distance_to_ship: -1, parent_position: position}
    }
    #[test]
    fn test_connected() {
        let mut network = Network::new();
        assert_eq!(network.is_connected(), true);
        network.add(new_node(CellIndex::new(0, 0, 0), TileType::Unset));
        assert_eq!(network.is_connected(), true);
        network.add(new_node(CellIndex::new(0, 0, 2), TileType::Unset));
        assert_eq!(network.is_connected(), false);
        network.add(new_node(CellIndex::new(0, 0, 1), TileType::Unset));
        assert_eq!(network.is_connected(), true);
    }

    #[test]
    fn test_is_adjacent() {
        let mut network = Network::new();
        let adjacent = CellIndex::new(0, 0, 0);
        let position = CellIndex::new(0, 0, 1);
        assert_eq!(network.is_adjacent(adjacent), false);
        network.add(new_node(position, TileType::MachineDrill));
        assert_eq!(network.is_adjacent(adjacent), true);
    }
}
