pub mod network;

use crate::screen::gui::format_units::format_unit;
use crate::world::map::cell::is_networkable;
use crate::world::map::{CellIndex, TileType};
use crate::world::networks::network::{Network, Node, Replacement};
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
        match self.replace_if_present(cell_index, new_machine) {
            Replacement::SplitNetwork | Replacement::Regular => {
                return true;
            }
            Replacement::Forbidden => {
                return false;
            }
            Replacement::None => {} // continue with addition
        }
        if !is_networkable(new_machine) {
            return true;
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
            return true;
        }
        if cell_index == self.ship_position {
            self.ship_network.add(node);
            return true;
        }
        // not connected to ship_network
        let adjacent_networks = self.get_adjacent_networks(cell_index);
        if adjacent_networks.len() > 0 {
            self.join_networks_and_add_node(node, &adjacent_networks);
        } else {
            self.add_new_network_with_node(node);
        };
        return true;
    }

    fn replace_if_present(&mut self, cell_index: CellIndex, new_machine: TileType) -> Replacement {
        let replacement = self
            .ship_network
            .replace_if_present(cell_index, new_machine);
        match replacement {
            Replacement::SplitNetwork => {
                let network_to_split = std::mem::take(&mut self.ship_network);
                self.re_add_network(network_to_split);
                return replacement;
            }
            Replacement::Regular => {
                return replacement;
            }
            Replacement::Forbidden => {
                return replacement;
            }
            Replacement::None => {}
        }
        for (i, network) in &mut self.unconnected_networks.iter_mut().enumerate() {
            let replacement = network.replace_if_present(cell_index, new_machine);
            match replacement {
                Replacement::SplitNetwork => {
                    self.split_network(i);
                    return replacement;
                }
                Replacement::Regular => {
                    if self.unconnected_networks.get(i).unwrap().len() == 0 {
                        self.unconnected_networks.remove(i);
                        // TODO: can this happen?
                    }
                    return replacement;
                }
                Replacement::Forbidden => {
                    return replacement;
                }
                Replacement::None => {}
            }
        }
        return Replacement::None;
    }

    fn re_add_network(&mut self, network_to_split: Network) {
        for node in network_to_split.nodes {
            self.add(node.position, node.tile);
        }
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

    fn join_networks_and_add_node(&mut self, node: Node, to_be_merged: &[usize]) {
        assert!(to_be_merged.len() > 0);
        let to_be_removed = &to_be_merged[1..];
        let kept = to_be_merged[0];
        let mut networks_to_be_merged = Vec::new();
        for i in to_be_removed.iter().rev() {
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
        self.air_cleaned += self.ship_network.update().air_cleaned;
        for network in self.unconnected_networks.iter_mut() {
            self.air_cleaned += network.update().air_cleaned;
        }
    }

    pub fn len(&self) -> usize {
        self.unconnected_networks.len() + 1 // plus the ship network
    }

    pub fn get_non_ship_machine_count(&self) -> i32 {
        let mut machines = 0;
        machines += self.ship_network.len() as i32 - 1;
        for network in &self.unconnected_networks {
            machines += network.len() as i32;
        }
        machines
    }

    pub fn iter(&self) -> impl Iterator<Item=&Network> {
        std::iter::once(&self.ship_network).chain(self.unconnected_networks.iter())
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
        self.re_add_network(network_to_split)
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
        networks.add(CellIndex::new(0, 0, 10), TileType::MachineAssembler);
        networks.add(CellIndex::new(0, 0, 12), TileType::MachineAssembler);
        assert_eq!(networks.len(), 3);
        networks.add(CellIndex::new(0, 0, 11), TileType::MachineAssembler);
        assert_eq!(networks.len(), 2);
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
        networks.add(CellIndex::new(0, 0, 10), TileType::MachineAssembler);
        networks.add(CellIndex::new(0, 0, 10), TileType::MachineDrill);
        assert_eq!(networks.len(), 2);
        assert_eq!(networks.get_non_ship_machine_count(), 1);
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
        networks.add(CellIndex::new(0, 0, 10), TileType::MachineAssembler);
        networks.add(CellIndex::new(0, 0, 11), TileType::MachineAssembler);
        networks.replace_if_present(CellIndex::new(0, 0, 11), TileType::FloorRock);
        assert_eq!(networks.len(), 2);
        assert_eq!(networks.unconnected_networks.get(0).unwrap().nodes.len(), 1);
        networks.replace_if_present(CellIndex::new(0, 0, 10), TileType::FloorRock);
        assert_eq!(networks.len(), 1);
    }

    #[test]
    fn test_split_network() {
        let mut networks = Networks::new_default();
        networks.add(CellIndex::new(0, 0, 10), TileType::MachineAssembler);
        networks.add(CellIndex::new(0, 0, 11), TileType::MachineAssembler);
        networks.add(CellIndex::new(0, 0, 12), TileType::MachineAssembler);
        assert_eq!(networks.len(), 2);
        networks.add(CellIndex::new(0, 0, 11), TileType::FloorRock);
        assert_eq!(networks.len(), 3);
    }

    #[test]
    fn test_air_cleaned_is_kept_when_spliting_network() {
        let expected_air_cleaned = 1.0;
        let mut networks = Networks::new_default();
        let ship = networks.ship_position;
        networks.add(ship + CellIndex::new(1, 0, 0), TileType::MachineSolarPanel);
        networks.add(ship + CellIndex::new(1, 1, 0), TileType::MachineAirCleaner);
        networks.update();
        assert_eq!(networks.get_total_air_cleaned(), expected_air_cleaned);
        networks.add(ship + CellIndex::new(1, 0, 2), TileType::MachineAssembler);
        assert_eq!(networks.get_total_air_cleaned(), expected_air_cleaned);
        networks.add(ship + CellIndex::new(1, 0, 1), TileType::MachineAssembler);
        assert_eq!(networks.get_total_air_cleaned(), expected_air_cleaned);
        networks.add(ship + CellIndex::new(1, 0, 1), TileType::FloorRock);
        assert_eq!(networks.get_total_air_cleaned(), expected_air_cleaned);
    }

    fn new_node(position: CellIndex, tile: TileType) -> Node {
        Node { position, tile }
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
