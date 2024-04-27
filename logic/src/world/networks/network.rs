use std::collections::{HashSet, VecDeque};

use crate::screen::gui::format_units::{
    format_grams, format_unit, format_watts, Grams, Liters, Watts,
};
use crate::world::map::cell::is_networkable;
use crate::world::map::{CellIndex, TileType};
use crate::world::robots::CellIndexDiff;

pub const POWER_PER_SOLAR_PANEL: Watts = 1000.0;
pub const POWER_CONSUMED_PER_MACHINE: Watts = POWER_PER_SOLAR_PANEL;

const AIR_CLEANED_PER_CLEANER_PER_UPDATE: Liters = 1.0;

pub const MATERIAL_NEEDED_FOR_A_MACHINE: Grams = 100_000.0;
pub const SPACESHIP_INITIAL_STORAGE: Grams = 500_000.0;
pub const MAX_STORAGE_PER_MACHINE: Grams = 1_000_000.0;
pub const MAX_STORAGE_PER_STORAGE_MACHINE: Grams = 10_000_000.0;
pub const WALL_WEIGHT: Grams = 10_000_000.0;

pub struct Network {
    pub nodes: Vec<Node>,
    pub stored_resources: Grams,
}

#[derive(Copy, Clone)]
pub struct Node {
    pub position: CellIndex,
    pub tile: TileType,
}

#[derive(PartialEq, Debug)]
pub enum Replacement {
    SplitNetwork,
    Regular,
    Forbidden,
    None,
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
            nodes: Vec::new(),
            stored_resources: 0.0,
        }
    }
    pub fn update(&mut self) -> NetworkUpdate {
        NetworkUpdate {
            air_cleaned: if self.is_power_satisfied() {
                let mut air_cleaners = 0;
                for node in &self.nodes {
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
        storage_count as f64 * MAX_STORAGE_PER_STORAGE_MACHINE
            + (self.len() as i32 - storage_count) as f64 * MAX_STORAGE_PER_MACHINE
    }
    pub fn get_storage_capacity_str(&self) -> String {
        format_grams(self.get_storage_capacity())
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
    pub fn get_node(&self, cell_index: CellIndex) -> Option<Node> {
        for node in &self.nodes {
            if node.position == cell_index {
                return Some(*node);
            }
        }
        None
    }

    pub fn replace_if_present(
        &mut self,
        cell_index: CellIndex,
        new_machine: TileType,
    ) -> Replacement {
        let mut index_to_change = Option::None;
        for (i, node) in self.nodes.iter().enumerate() {
            if node.position == cell_index {
                if node.tile == TileType::MachineShip {
                    return Replacement::Forbidden;
                }
                index_to_change = Option::Some(i);
                break;
            }
        }
        return if let Option::Some(i) = index_to_change {
            let old_material_regained = material_composition(self.nodes[i].tile);
            let new_material_spent = material_composition(new_machine);
            let future_storage =
                self.get_stored_resources() + old_material_regained - new_material_spent;
            let future_capacity = self.get_storage_capacity() + storage_capacity(new_machine)
                - storage_capacity(self.nodes[i].tile);
            if future_storage > future_capacity {
                return Replacement::Forbidden;
            } else if future_storage < 0.0 {
                return Replacement::Forbidden;
            }
            self.stored_resources += old_material_regained;
            self.stored_resources -= new_material_spent;
            let replacement_is_really_a_removal = !is_networkable(new_machine);
            if replacement_is_really_a_removal {
                self.nodes.remove(i);
                if self.is_connected() {
                    Replacement::Regular
                } else {
                    Replacement::SplitNetwork
                }
            } else {
                self.nodes.get_mut(i).unwrap().tile = new_machine;
                Replacement::Regular
            }
        } else {
            Replacement::None
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
        if node.tile == TileType::MachineShip {
            self.stored_resources += SPACESHIP_INITIAL_STORAGE;
        } else {
            self.stored_resources -= material_composition(node.tile);
        }
    }

    pub fn join(&mut self, other: Network) {
        for node in other.nodes {
            self.add(node);
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
            MATERIAL_NEEDED_FOR_A_MACHINE
        }
    }
}
pub fn storage_capacity(tile: TileType) -> Grams {
    match tile {
        TileType::Unset => {
            panic!("should not be asking the amount of capacity of an Unset tile")
        }
        TileType::WallRock | TileType::WallDirt => WALL_WEIGHT,
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
        | TileType::MachineSolarPanel
        | TileType::MachineShip => MAX_STORAGE_PER_MACHINE,
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
