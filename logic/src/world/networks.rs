pub mod network;

use crate::screen::gui::format_units::{format_liters, Grams};
use crate::world::map::cell::is_networkable;
use crate::world::map::transform_cells::TransformationFailure;
use crate::world::map::{CellIndex, TileType};
use crate::world::networks::network::{
    Addition, Network, Node, Replacement, MATERIAL_NEEDED_FOR_A_MACHINE, SPACESHIP_INITIAL_STORAGE,
};

pub struct Networks {
    ship_position: CellIndex,
    ship_network: Network,
    unconnected_networks: Vec<Network>,
    air_cleaned: f64,
}

impl Networks {
    pub fn new(ship_position: CellIndex) -> Self {
        let mut network = Network::new_with_storage(SPACESHIP_INITIAL_STORAGE);
        network.add_or_panic(
            Node {
                position: ship_position,
                tile: TileType::MachineShip,
            },
            TileType::Air,
        );
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

    pub fn add(
        &mut self,
        cell_index: CellIndex,
        new_machine: TileType,
        old_tile: TileType,
    ) -> bool {
        self.add_with_storage(cell_index, new_machine, old_tile, &mut 0.0)
    }
    pub fn add_with_reason(
        &mut self,
        cell_index: CellIndex,
        new_machine: TileType,
        old_tile: TileType,
    ) -> Option<TransformationFailure> {
        self.add_with_storage_with_reason(cell_index, new_machine, old_tile, &mut 0.0)
    }

    pub fn add_with_storage(
        &mut self,
        cell_index: CellIndex,
        new_machine: TileType,
        old_tile: TileType,
        storage: &mut Grams,
    ) -> bool {
        self.add_with_storage_with_reason(cell_index, new_machine, old_tile, storage)
            .is_none()
    }

    pub fn add_with_storage_with_reason(
        &mut self,
        cell_index: CellIndex,
        new_machine: TileType,
        old_tile: TileType,
        storage: &mut Grams,
    ) -> Option<TransformationFailure> {
        let replacement = self.replace_if_present_with_storage(cell_index, new_machine);
        match replacement {
            Replacement::SplitNetwork
            | Replacement::Ok
            | Replacement::Forbidden
            | Replacement::NotEnoughMaterial
            | Replacement::NotEnoughStorage => {
                return replacement.into();
            }
            Replacement::None => {} // continue with addition
        }

        let node = Node {
            position: cell_index,
            tile: new_machine,
        };
        if self.ship_network.is_adjacent(cell_index) {
            let addition = self.ship_network.try_add(node, old_tile);
            match addition {
                Addition::Ok => {
                    let adjacent_networks = self.get_adjacent_networks(cell_index);
                    for i_network in adjacent_networks.iter().rev() {
                        let joining_network = self.unconnected_networks.remove(*i_network);
                        self.re_add_network(joining_network);
                    }
                    *storage = self.ship_network.try_add_resources(*storage);
                }
                Addition::NotEnoughMaterial | Addition::NotEnoughStorage => {}
            };
            return addition.into();
        }
        if cell_index == self.ship_position {
            self.ship_network.add(node);
            *storage = self.ship_network.try_add_resources(*storage);
            return None;
        }
        // not connected to ship_network
        let adjacent_networks = self.get_adjacent_networks(cell_index);
        if adjacent_networks.len() > 0 {
            self.join_networks_and_add_node(node, old_tile, &adjacent_networks, storage);
        } else {
            if is_networkable(node.tile) {
                let addition = self.add_new_network_with_node(node, old_tile, storage);
                return addition.into();
            } else {
                return None;
            }
        };
        return None;
    }

    #[cfg(test)]
    fn replace_if_present(&mut self, cell_index: CellIndex, new_machine: TileType) -> Replacement {
        self.replace_if_present_with_storage(cell_index, new_machine)
    }
    fn replace_if_present_with_storage(
        &mut self,
        cell_index: CellIndex,
        new_machine: TileType,
    ) -> Replacement {
        let replacement = self
            .ship_network
            .replace_if_present(cell_index, new_machine);
        match replacement {
            Replacement::SplitNetwork
            | Replacement::Ok
            | Replacement::Forbidden
            | Replacement::NotEnoughStorage
            | Replacement::NotEnoughMaterial => {
                return replacement;
            }
            Replacement::None => {}
        }
        for (i, network) in &mut self.unconnected_networks.iter_mut().enumerate() {
            let replacement = network.replace_if_present(cell_index, new_machine);
            match replacement {
                Replacement::Ok => {
                    if self.unconnected_networks.get(i).unwrap().len() == 0 {
                        self.unconnected_networks.remove(i);
                        // TODO: can this happen?
                    }
                    return replacement;
                }
                Replacement::SplitNetwork
                | Replacement::Forbidden
                | Replacement::NotEnoughStorage
                | Replacement::NotEnoughMaterial => {
                    return replacement;
                }
                Replacement::None => {}
            }
        }
        return Replacement::None;
    }

    fn re_add_network(&mut self, network_to_split: Network) {
        let storage_to_redistribute = network_to_split.stored_resources;
        let mut storage_per_node = storage_to_redistribute;
        for node in network_to_split.nodes {
            storage_per_node += MATERIAL_NEEDED_FOR_A_MACHINE; // assumes the network already paid for this
            if !self.add_with_storage(
                node.position,
                node.tile,
                TileType::Air,
                &mut storage_per_node,
            ) {
                println!("Bug: Can not split network");
            }
        }
        if storage_per_node > 0.0 {
            println!("Bug: not enough capacity");
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

    fn join_networks_and_add_node(
        &mut self,
        node: Node,
        old_tile: TileType,
        to_be_merged: &[usize],
        storage: &mut Grams,
    ) {
        assert!(to_be_merged.len() > 0);
        let to_be_removed = &to_be_merged[1..];
        let kept = to_be_merged[0];
        let mut networks_to_be_merged = Vec::new();
        for i in to_be_removed.iter().rev() {
            networks_to_be_merged.push(self.unconnected_networks.remove(*i));
        }
        let network_kept = self.unconnected_networks.get_mut(kept).unwrap();
        *storage = network_kept.try_add_resources(*storage);
        network_kept.add_or_panic(node, old_tile);
        while let Option::Some(network_to_be_merged) = networks_to_be_merged.pop() {
            network_kept.join(network_to_be_merged);
        }
        *storage = network_kept.try_add_resources(*storage);
    }

    fn add_new_network_with_node(
        &mut self,
        node: Node,
        old_tile: TileType,
        storage: &mut Grams,
    ) -> Addition {
        let mut network = Network::new_with_storage(*storage);
        let addition = network.try_add(node, old_tile);
        if network.stored_resources > network.get_storage_capacity() {
            return Addition::NotEnoughStorage;
        }
        *storage = 0.0;
        if addition == Addition::Ok {
            self.unconnected_networks.push(network);
        }
        addition
    }

    pub fn update(&mut self) {
        let mut air_cleaned = 0.0;
        for network in self.iter_mut() {
            air_cleaned += network.update().air_cleaned;
        }
        self.air_cleaned += air_cleaned;
    }

    pub fn len(&self) -> usize {
        self.unconnected_networks.len() + 1 // plus the ship network
    }

    pub fn get_non_ship_machine_count(&self) -> i32 {
        let mut machines = 0;
        for network in self.iter() {
            machines += network.len() as i32;
        }
        machines - 1
    }

    pub fn iter(&self) -> impl Iterator<Item = &Network> {
        std::iter::once(&self.ship_network).chain(self.unconnected_networks.iter())
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut Network> {
        std::iter::once(&mut self.ship_network).chain(self.unconnected_networks.iter_mut())
    }

    pub fn get_total_air_cleaned(&self) -> f64 {
        self.air_cleaned
    }

    pub fn get_total_air_cleaned_str(&self) -> String {
        let air_cleaned = self.get_total_air_cleaned();
        format_liters(air_cleaned)
    }
    pub fn get_stored_resources(&self) -> Grams {
        self.iter()
            .map(|n| n.get_stored_resources())
            .reduce(|a, b| a + b)
            .unwrap_or(0.0)
    }
    pub fn get_storage_capacity(&self) -> Grams {
        self.iter()
            .map(|n| n.get_storage_capacity())
            .reduce(|a, b| a + b)
            .unwrap_or(0.0)
    }

    pub fn reset_production(&mut self) {
        self.air_cleaned = 0.0;
    }

    #[cfg(test)]
    pub fn set_production(&mut self, air_cleaned: f64) {
        self.air_cleaned = air_cleaned;
    }

    pub fn clear(&mut self) {
        *self = Self::new(self.ship_position)
    }

    pub fn get(&self, position: CellIndex) -> Option<Node> {
        let node_opt = (&self.ship_network).get_node(position);
        if node_opt.is_some() {
            return node_opt;
        }
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
    use crate::world::map::TileType::{MachineStorage, WallRock, Wire};
    use crate::world::map::{CellIndex, TileType};
    use crate::world::networks::network::WALL_WEIGHT;
    use TileType::{Air, MachineAirCleaner, MachineAssembler};

    #[test]
    fn test_join_networks() {
        let mut networks = Networks::new_default();
        assert_eq!(
            networks.add(CellIndex::new(0, 0, 1), MachineAssembler, Air),
            true
        );
        let mut material_for_separate_network = MATERIAL_NEEDED_FOR_A_MACHINE;
        assert_eq!(
            networks.add_with_storage(
                CellIndex::new(0, 0, 3),
                MachineAssembler,
                Air,
                &mut material_for_separate_network
            ),
            true
        );
        assert_eq!(networks.len(), 2);
        networks.add(CellIndex::new(0, 0, 2), MachineAssembler, Air);
        assert_eq!(networks.len(), 1);
        assert_eq!(networks.get_non_ship_machine_count(), 3);
    }

    #[test]
    fn test_split_and_join_networks_keeps_storage() {
        let mut networks = Networks::new_default();

        let resources_before_constructing = networks.get_stored_resources();
        assert_eq!(
            networks.add(CellIndex::new(0, 0, 1), MachineStorage, WallRock),
            true
        );
        assert_eq!(networks.add(CellIndex::new(0, 0, 2), Wire, Air), true);
        assert_eq!(networks.add(CellIndex::new(0, 0, 3), Wire, Air), true);

        let resources_after_constructing = networks.get_stored_resources();
        assert_eq!(networks.add(CellIndex::new(0, 0, 2), Air, Air), false);
        assert_eq!(
            networks.get_stored_resources(),
            resources_after_constructing
        );

        assert_eq!(networks.add(CellIndex::new(0, 0, 3), Air, Air), true);
        assert_eq!(networks.add(CellIndex::new(0, 0, 2), Air, Air), true);
        assert_eq!(networks.add(CellIndex::new(0, 0, 1), WallRock, Air), true);
        assert_eq!(
            networks.get_stored_resources(),
            resources_before_constructing
        );

        assert_eq!(
            resources_after_constructing - resources_before_constructing,
            WALL_WEIGHT - 3.0 * MATERIAL_NEEDED_FOR_A_MACHINE
        )
    }

    #[test]
    fn test_append_to_network() {
        let mut networks = Networks::new_default();
        networks.add(CellIndex::new(0, 0, 0), MachineAssembler, Air);
        networks.add(CellIndex::new(0, 0, 1), MachineAssembler, Air);
        assert_eq!(networks.len(), 1);
    }

    #[test]
    fn test_replace_machine() {
        let mut networks = Networks::new_default();
        assert_eq!(
            networks.add(CellIndex::new(0, 0, 1), MachineAssembler, Air),
            true
        );
        assert_eq!(
            networks.add(CellIndex::new(0, 0, 1), TileType::MachineDrill, Air),
            true
        );
        assert_eq!(networks.len(), 1);
        assert_eq!(networks.get_non_ship_machine_count(), 1);
        assert_eq!(
            networks.ship_network.nodes.last().unwrap().tile,
            TileType::MachineDrill
        );
    }

    #[test]
    fn test_destroy_machine() {
        let mut networks = Networks::new_default();
        assert_eq!(
            networks.add(CellIndex::new(0, 0, 1), MachineAssembler, Air),
            true
        );
        assert_eq!(
            networks.add(CellIndex::new(0, 0, 2), MachineAssembler, Air),
            true
        );
        assert_eq!(
            networks.replace_if_present(CellIndex::new(0, 0, 1), Air),
            Replacement::SplitNetwork
        );
        assert_eq!(networks.len(), 1);
        networks.replace_if_present(CellIndex::new(0, 0, 2), Air);
        networks.replace_if_present(CellIndex::new(0, 0, 1), Air);
        assert_eq!(networks.len(), 1);
        assert_eq!(networks.get_non_ship_machine_count(), 0);
    }

    #[test]
    fn test_split_network() {
        let mut networks = Networks::new_default();
        assert_eq!(
            networks.add(CellIndex::new(0, 0, 10), MachineStorage, WallRock),
            true
        );
        assert_eq!(
            networks.add(CellIndex::new(0, 0, 11), MachineAssembler, Air),
            true
        );
        assert_eq!(
            networks.add(CellIndex::new(0, 0, 12), MachineAssembler, Air),
            true
        );
        assert_eq!(networks.len(), 2);
        assert_eq!(networks.get_non_ship_machine_count(), 3);
        assert_eq!(
            networks.add(CellIndex::new(0, 0, 11), TileType::Air, Air),
            false
        );
        assert_eq!(networks.len(), 2);
    }

    #[test]
    fn test_air_cleaned_is_kept_when_spliting_network() {
        let expected_air_cleaned = 1.0;
        let mut networks = Networks::new_default();
        let ship = networks.ship_position;
        networks.add(
            ship + CellIndex::new(1, 0, 0),
            TileType::MachineSolarPanel,
            Air,
        );
        networks.add(ship + CellIndex::new(1, 1, 0), MachineAirCleaner, Air);
        networks.update();
        assert_eq!(networks.get_total_air_cleaned(), expected_air_cleaned);
        networks.add(ship + CellIndex::new(1, 0, 2), MachineAssembler, Air);
        assert_eq!(networks.get_total_air_cleaned(), expected_air_cleaned);
        networks.add(ship + CellIndex::new(1, 0, 1), MachineAssembler, Air);
        assert_eq!(networks.get_total_air_cleaned(), expected_air_cleaned);
        networks.add(ship + CellIndex::new(1, 0, 1), TileType::WallRock, Air);
        assert_eq!(networks.get_total_air_cleaned(), expected_air_cleaned);
    }

    fn new_node(position: CellIndex, tile: TileType) -> Node {
        Node { position, tile }
    }
    #[test]
    fn test_connected() {
        let mut network = Network::new();
        assert_eq!(network.is_connected(), true);
        network.add(new_node(CellIndex::new(0, 0, 0), MachineAirCleaner));
        assert_eq!(network.is_connected(), true);
        network.add(new_node(CellIndex::new(0, 0, 2), MachineAirCleaner));
        assert_eq!(network.is_connected(), false);
        network.add(new_node(CellIndex::new(0, 0, 1), MachineAirCleaner));
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

#[cfg(test)]
mod storage_tests {
    use super::*;
    use crate::world::map::TileType::WallRock;
    use crate::world::networks::network::{
        MAX_STORAGE_PER_SPACESHIP, SPACESHIP_INITIAL_STORAGE, WALL_WEIGHT,
    };
    use TileType::{Air, MachineStorage, Wire};

    #[test]
    fn test_initial_storage_capacity() {
        let networks = Networks::new_default();
        assert_eq!(
            networks.get_stored_resources(),
            SPACESHIP_INITIAL_STORAGE - MATERIAL_NEEDED_FOR_A_MACHINE
        );
        assert_eq!(networks.get_storage_capacity(), MAX_STORAGE_PER_SPACESHIP);
    }
    #[test]
    fn test_can_not_add_machine_without_resources() {
        let mut networks = Networks::new(CellIndex::new(0, 0, 0));
        assert_eq!(networks.add(CellIndex::new(0, 0, 1), Wire, Air), true);
        assert_eq!(networks.add(CellIndex::new(0, 0, 2), Wire, Air), true);
        assert_eq!(networks.add(CellIndex::new(0, 0, 3), Wire, Air), true);
        assert_eq!(networks.add(CellIndex::new(0, 0, 4), Wire, Air), true);
        assert_eq!(networks.add(CellIndex::new(0, 0, 5), Wire, Air), false);
    }

    #[test]
    fn test_can_not_add_machine_without_storage_capacity() {
        let mut networks = Networks::new(CellIndex::new(0, 0, 0));
        assert_eq!(networks.add(CellIndex::new(0, 0, 1), Wire, WallRock), false);
    }
    #[test]
    fn test_can_not_add_machine_if_that_makes_storage_capacity_insufficient() {
        let mut networks = Networks::new(CellIndex::new(0, 0, 0));
        assert_eq!(
            networks.add(CellIndex::new(0, 0, 1), MachineStorage, WallRock),
            true
        );
        assert_eq!(networks.add(CellIndex::new(0, 0, 1), Wire, WallRock), false);
    }

    #[test]
    fn test_adding_storage_without_capacity_is_allowed() {
        let mut networks = Networks::new_default();
        assert_eq!(
            networks.add(CellIndex::new(0, 0, 1), MachineStorage, WallRock),
            true
        );
    }

    #[test]
    fn test_remove_machine_gives_resources_back() {
        let mut networks = Networks::new(CellIndex::new(0, 0, 0));
        let material_before_constructing = networks.get_stored_resources();
        assert_eq!(networks.add(CellIndex::new(0, 0, 1), Wire, Air), true);
        assert_ne!(
            networks.get_stored_resources(),
            material_before_constructing
        );
        assert_eq!(networks.add(CellIndex::new(0, 0, 1), Air, Wire), true);
        assert_eq!(
            networks.get_stored_resources(),
            material_before_constructing
        );
    }

    #[test]
    fn test_convert_wall_and_air_to_storage() {
        let mut networks = Networks::new(CellIndex::new(0, 0, 0));
        assert_eq!(
            networks.add(CellIndex::new(0, 0, 1), MachineStorage, Air),
            true
        );
        let material_before_constructing = networks.get_stored_resources();
        assert_eq!(
            networks.add(CellIndex::new(0, 0, 2), MachineStorage, WallRock),
            true
        );
        assert_eq!(
            networks.get_stored_resources(),
            material_before_constructing + WALL_WEIGHT - MATERIAL_NEEDED_FOR_A_MACHINE
        );

        let material_before_constructing = networks.get_stored_resources();
        assert_eq!(
            networks.add(CellIndex::new(0, 0, 3), MachineStorage, Air),
            true
        );
        assert_eq!(
            networks.get_stored_resources(),
            material_before_constructing - MATERIAL_NEEDED_FOR_A_MACHINE
        );
    }

    #[test]
    fn test_build_wall_without_enough_material() {
        let mut networks = Networks::new(CellIndex::new(0, 0, 0));
        assert_eq!(networks.add(CellIndex::new(1, 0, 0), WallRock, Air), false); // not enough material
        assert_eq!(
            networks.add(CellIndex::new(0, 0, 1), MachineStorage, WallRock),
            true
        );
        let material_before_building = networks.get_stored_resources();
        assert_eq!(networks.add(CellIndex::new(1, 0, 0), WallRock, Air), true);
        assert_eq!(
            networks.get_stored_resources(),
            material_before_building - WALL_WEIGHT
        )
    }
}
