use std::collections::{HashMap, HashSet, VecDeque};

use crate::screen::gui::format_units::{
    format_grams, format_unit, format_watts, Grams, Liters, Watts,
};
use crate::world::map::cell::is_networkable;
use crate::world::map::{CellIndex, TileType};
use crate::world::robots::CellIndexDiff;

pub const POWER_PER_SOLAR_PANEL: Watts = 1000.0;
pub const POWER_CONSUMED_PER_MACHINE: Watts = POWER_PER_SOLAR_PANEL;

const AIR_CLEANED_PER_CLEANER_PER_UPDATE: Liters = 1.0;

pub const MATERIAL_NEEDED_FOR_A_MACHINE: Grams = 1_000_000.0;
pub const MAX_STORAGE_PER_SPACESHIP: Grams = 5.0 * MATERIAL_NEEDED_FOR_A_MACHINE;
pub const SPACESHIP_INITIAL_STORAGE: Grams = 5.0 * MATERIAL_NEEDED_FOR_A_MACHINE;
// pub const MAX_STORAGE_PER_MACHINE: Grams = 1_000_000.0;
pub const MAX_STORAGE_PER_MACHINE: Grams = 0.0;
pub const MAX_STORAGE_PER_STORAGE_MACHINE: Grams = 10_000_000.0;
pub const WALL_WEIGHT: Grams = 10_000_000.0;

// pub type Nodes = Vec<Node>;
pub type Nodes = HashMap<CellIndex, TileType>;

#[derive(Debug)]
pub struct Network {
    pub nodes: Nodes,
    pub stored_resources: Grams,
}

#[derive(Copy, Clone, Debug)]
pub struct Node {
    pub position: CellIndex,
    pub tile: TileType,
}

#[derive(PartialEq, Debug)]
pub enum Replacement {
    Ok,
    SplitNetwork,
    Forbidden,
    NotEnoughMaterial,
    NotEnoughStorage,
    None,
}

#[derive(PartialEq, Debug)]
pub enum Addition {
    Ok,
    NotEnoughMaterial,
    NotEnoughStorage,
}

pub struct NetworkUpdate {
    pub air_cleaned: Liters,
}

// pub struct NetworkEffect {
//     power: f64,
//     storage: f64,
// }

impl Network {
    pub fn new() -> Self {
        Network {
            nodes: Nodes::new(),
            stored_resources: 0.0,
        }
    }
    pub fn new_with_storage(initial_storage: Grams) -> Self {
        Network {
            nodes: Nodes::new(),
            stored_resources: initial_storage,
        }
    }
    pub fn update(&mut self) -> NetworkUpdate {
        NetworkUpdate {
            air_cleaned: if self.is_power_satisfied() {
                let mut air_cleaners = 0;
                for node in self.nodes() {
                    match node.tile {
                        TileType::MachineAirCleaner => air_cleaners += 1,
                        TileType::MachineAssembler => {}
                        _ => {}
                    }
                }
                air_cleaners as f64 * AIR_CLEANED_PER_CLEANER_PER_UPDATE
            } else {
                0.0
            },
        }
    }

    pub fn nodes(&self) -> impl Iterator<Item = Node> + '_ {
        self.nodes.iter().map(|(pos, tile)| Node::new(*pos, *tile))
    }
    pub fn len(&self) -> usize {
        self.nodes.len()
    }

    pub fn get_power_generated_str(&self) -> String {
        let power = self.get_power_generated();
        format_watts(power)
    }

    fn get_power_generated(&self) -> f64 {
        let solar_panels_count = self.count_tiles_of_type_in(&[TileType::MachineSolarPanel]);
        let power = solar_panels_count as f64 * POWER_PER_SOLAR_PANEL;
        power
    }

    pub fn get_power_required_str(&self) -> String {
        let power = self.get_power_required();
        format_watts(power)
    }

    fn get_power_required(&self) -> f64 {
        let machines_count = self.count_tiles_of_type_in(&[
            TileType::MachineDrill,
            TileType::MachineAssembler,
            TileType::MachineAirCleaner,
            TileType::MachineStorage,
        ]);
        let power = machines_count as f64 * POWER_CONSUMED_PER_MACHINE;
        power
    }

    pub fn is_power_satisfied(&self) -> bool {
        self.get_power_generated() >= self.get_power_required()
    }

    fn count_tiles_of_type_in(&self, tiles: &[TileType]) -> i32 {
        let mut count = 0;
        for node in self.nodes() {
            if tiles.contains(&node.tile) {
                count += 1;
            }
        }
        count
    }

    fn get_air_cleaned_speed(&self) -> f64 {
        if self.is_power_satisfied() {
            let air_cleaners_count = self.count_tiles_of_type_in(&[TileType::MachineAirCleaner]);
            let liters = air_cleaners_count as f64 * f64::from(AIR_CLEANED_PER_CLEANER_PER_UPDATE);
            liters
        } else {
            0.0
        }
    }

    pub fn get_air_cleaned_speed_str(&self) -> String {
        let air_cleaned = self.get_air_cleaned_speed();
        format_unit(air_cleaned, "L/s")
    }

    pub fn get_stored_resources(&self) -> Grams {
        self.stored_resources
    }

    pub fn get_stored_resources_str(&self) -> String {
        format_grams(self.get_stored_resources())
    }

    pub fn get_storage_capacity(&self) -> Grams {
        let storage_count = self.count_tiles_of_type_in(&[TileType::MachineStorage]);
        let ships = self.count_tiles_of_type_in(&[TileType::MachineShip]);
        storage_count as f64 * MAX_STORAGE_PER_STORAGE_MACHINE
            + ships as f64 * MAX_STORAGE_PER_SPACESHIP
            + (self.len() as i32 - storage_count - ships) as f64 * MAX_STORAGE_PER_MACHINE
    }
    pub fn get_storage_capacity_str(&self) -> String {
        format_grams(self.get_storage_capacity())
    }
    pub fn try_add_resources(&mut self, resources: Grams) -> Grams {
        self.stored_resources += resources;
        let overflow = self.stored_resources - self.get_storage_capacity();
        if overflow > 0.0 {
            self.stored_resources -= overflow;
            return overflow;
        } else {
            return 0.0;
        }
    }
    #[allow(unused)]
    fn get(&mut self, cell_index: CellIndex) -> Option<&mut TileType> {
        self.nodes.get_mut(&cell_index)
        // for node in &mut self.nodes() {
        //     if node.position == cell_index {
        //         return Option::Some(&mut node.tile);
        //     }
        // }
        // Option::None
    }
    pub fn get_node(&self, cell_index: CellIndex) -> Option<Node> {
        self.nodes
            .get(&cell_index)
            .map(|tile| Node::new(cell_index, *tile))
        // for node in self.nodes() {
        //     if node.position == cell_index {
        //         return Some(node);
        //     }
        // }
        // None
    }

    pub fn replace_if_present(
        &mut self,
        cell_index: CellIndex,
        new_machine: TileType,
    ) -> Replacement {
        return if let Option::Some(node) = self.get_node(cell_index) {
            let old_tile = node.tile;
            if old_tile == TileType::MachineShip {
                return Replacement::Forbidden;
            }
            let (old_material_regained, new_material_spent, future_storage, future_capacity) =
                Self::predict_storage(new_machine, old_tile, self);
            if future_storage > future_capacity {
                return Replacement::NotEnoughStorage;
            } else if future_storage < 0.0 {
                return Replacement::NotEnoughMaterial;
            }
            let replacement_is_really_a_removal = !is_networkable(new_machine);
            if replacement_is_really_a_removal {
                let removed = self.nodes.remove(&cell_index).unwrap();
                if !self.is_connected() {
                    self.nodes.insert(cell_index, removed);
                    return Replacement::SplitNetwork;
                }
            } else {
                *self.nodes.get_mut(&cell_index).unwrap() = new_machine;
            }
            self.stored_resources += old_material_regained;
            self.stored_resources -= new_material_spent;
            Replacement::Ok
        } else {
            Replacement::None
        };
    }

    pub fn predict_storage(
        new_machine: TileType,
        old_tile: TileType,
        network: &Network,
    ) -> (Grams, Grams, Grams, Grams) {
        let old_material_regained = material_composition(old_tile);
        let new_material_spent = material_composition(new_machine);
        let extra_storage_in_ship =// if new_machine == TileType::MachineShip { // The ship storage is added when creating a network
            // SPACESHIP_INITIAL_STORAGE
        // } else {
            0.0
        // }
    ;
        let future_storage = network.get_stored_resources() + old_material_regained
            - new_material_spent
            + extra_storage_in_ship;
        let future_capacity = network.get_storage_capacity() + storage_capacity(new_machine)
            - storage_capacity(old_tile);
        (
            old_material_regained,
            new_material_spent,
            future_storage,
            future_capacity,
        )
    }

    pub fn is_adjacent(&self, cell_index: CellIndex) -> bool {
        for neighbour in neighbours(cell_index) {
            if self.get_node(neighbour).is_some() {
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
        let first_position = self.nodes.iter().next().unwrap().0;
        queue.push_back(*first_position);
        reachable.insert(*first_position);
        while let Some(position) = queue.pop_front() {
            let neighbours = neighbours(position);
            let max_neighbours = neighbours.len();
            let mut neighbours_found = 0;
            for neighbour in neighbours {
                if self.get_node(neighbour).is_some() && !reachable.contains(&neighbour) {
                    queue.push_back(neighbour);
                    reachable.insert(neighbour);
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
        if is_networkable(node.tile) {
            self.add_node(node);
        }
        self.stored_resources -= material_composition(node.tile);
    }
    pub fn add_no_spend(&mut self, node: Node) {
        if is_networkable(node.tile) {
            self.add_node(node);
        }
    }
    fn add_node(&mut self, node: Node) {
        self.nodes.insert(node.position, node.tile);
    }

    pub fn try_add(&mut self, node: Node, old_tile: TileType) -> Addition {
        let (old_material_regained, new_material_spent, future_storage, future_capacity) =
            Self::predict_storage(node.tile, old_tile, self);
        if future_storage < 0.0 {
            Addition::NotEnoughMaterial
        } else if future_storage > future_capacity {
            Addition::NotEnoughStorage
        } else {
            if is_networkable(node.tile) {
                self.add_node(node);
            }
            self.stored_resources -= new_material_spent;
            self.stored_resources += old_material_regained;
            Addition::Ok
        }
    }
    pub fn add_or_panic(&mut self, node: Node, old_tile: TileType) {
        let addition = self.try_add(node, old_tile);
        if addition != Addition::Ok {
            panic!(
                "failed to add node to network ({:?}). Node: {:?}, old_tile: {:?}, Network: {:?}",
                addition, node, old_tile, &self
            );
        }
    }

    pub fn only_add(&mut self, node: Node) {
        self.add_node(node)
    }

    pub fn join(&mut self, other: Network) {
        for node in other.nodes() {
            self.add_no_spend(node);
        }
    }
}

pub fn material_composition(tile: TileType) -> Grams {
    match tile {
        TileType::Unset => {
            panic!("should not be asking the amount of material of an Unset tile")
        }
        TileType::WallRock | TileType::WallDirt => WALL_WEIGHT,
        TileType::FloorRock | TileType::FloorDirt => {
            panic!("floor is deprecated, should not be asking amount of material")
        }
        TileType::Stairs => {
            panic!("stairs are deprecated, should not be asking amount of material")
        }
        TileType::Air => 0.0,
        TileType::Wire
        | TileType::MachineAssembler
        | TileType::MachineAirCleaner
        | TileType::MachineDrill
        | TileType::MachineSolarPanel
        | TileType::MachineShip
        | TileType::MachineStorage => MATERIAL_NEEDED_FOR_A_MACHINE,
        TileType::TreeHealthy | TileType::TreeSparse | TileType::TreeDying | TileType::TreeDead => {
            0.0
        }
    }
}
pub fn storage_capacity(tile: TileType) -> Grams {
    match tile {
        TileType::Unset => {
            panic!("should not be asking the amount of capacity of an Unset tile")
        }
        TileType::WallRock | TileType::WallDirt => 0.0,
        TileType::FloorRock | TileType::FloorDirt => {
            panic!("floor is deprecated, should not be asking amount of capacity")
        }
        TileType::Stairs => {
            panic!("stairs are deprecated, should not be asking amount of capacity")
        }
        TileType::Air => 0.0,
        TileType::Wire
        | TileType::MachineAssembler
        | TileType::MachineAirCleaner
        | TileType::MachineDrill
        | TileType::MachineSolarPanel => MAX_STORAGE_PER_MACHINE,
        TileType::MachineShip => MAX_STORAGE_PER_SPACESHIP,
        TileType::MachineStorage => MAX_STORAGE_PER_STORAGE_MACHINE,
        TileType::TreeHealthy | TileType::TreeSparse | TileType::TreeDying | TileType::TreeDead => {
            0.0
        }
    }
}

impl Default for Network {
    fn default() -> Self {
        Self::new()
    }
}

impl Node {
    pub fn new(position: CellIndex, tile: TileType) -> Self {
        Self { position, tile }
    }
}
pub fn is_adjacent(a: CellIndex, b: CellIndex) -> bool {
    let diff: CellIndexDiff = a - b;
    adjacent_positions().contains(&diff)
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
pub fn neighbours(a: CellIndex) -> [CellIndexDiff; 6] {
    [
        a + CellIndexDiff::new(1, 0, 0),
        a + CellIndexDiff::new(-1, 0, 0),
        a + CellIndexDiff::new(0, 1, 0),
        a + CellIndexDiff::new(0, -1, 0),
        a + CellIndexDiff::new(0, 0, 1),
        a + CellIndexDiff::new(0, 0, -1),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::world::map::CellCubeIterator;

    #[test]
    fn benchmark_is_connected() {
        let mut network = Network::new();
        let cube_side = 6;
        let start = -cube_side / 2;
        let end = start + cube_side - 1;
        let start_cell = CellIndex::new(start, start, start);
        let end_cell = CellIndex::new(end, end, end);
        let iter = CellCubeIterator::new(start_cell, end_cell);
        for cell_index in iter {
            network.only_add(Node::new(cell_index, TileType::MachineStorage));
        }
        let iter = CellCubeIterator::new(start_cell, end_cell);
        let mut removed = 0;
        let mut replacements = 0;
        let start = std::time::Instant::now();

        for cell_index in iter {
            let replacement = network.replace_if_present(cell_index, TileType::Air);
            if replacement == Replacement::Ok {
                removed += 1;
            }
            replacements += 1;
        }
        let elapsed = start.elapsed();
        println!(
            "elapsed removing {}/{} cells: {} µs",
            removed,
            replacements,
            elapsed.as_micros()
        );
    }
}
