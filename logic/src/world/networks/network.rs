use crate::screen::gui::format_units::{format_unit, Grams, Liters, Watts};
use crate::world::map::cell::is_networkable;
use crate::world::map::{CellIndex, TileType};
use crate::world::robots::CellIndexDiff;
use std::collections::{HashSet, VecDeque};

pub struct Network {
    pub nodes: Vec<Node>,
}

#[derive(Copy, Clone)]
pub struct Node {
    pub position: CellIndex,
    pub tile: TileType,
}


// pub const POWER_PER_SOLAR_PANEL: Watts = Watts(1000.0);
// pub const POWER_PER_SOLAR_PANEL: Watts = 1000.0;
pub const POWER_PER_SOLAR_PANEL: Watts = Watts { quantity: 1000.0 };
// pub const POWER_CONSUMED_PER_MACHINE: Watts = Watts(POWER_PER_SOLAR_PANEL.0);
// pub const POWER_CONSUMED_PER_MACHINE: Watts = POWER_PER_SOLAR_PANEL;
pub const POWER_CONSUMED_PER_MACHINE: Watts = POWER_PER_SOLAR_PANEL;

// const AIR_CLEANED_PER_CLEANER_PER_UPDATE: Liters = Liters(1.0);
// const AIR_CLEANED_PER_CLEANER_PER_UPDATE: Liters = 1.0;
const AIR_CLEANED_PER_CLEANER_PER_UPDATE: Liters = Liters {quantity: 1.0};

// pub const STORAGE_PER_MACHINE: Grams = 1_000_000.0;
// pub const STORAGE_PER_STORAGE_MACHINE: Grams = 10_000_000.0;
// const STORAGE_PER_WALL: Grams = 100_000_000.0;
pub const STORAGE_PER_MACHINE: Grams = Grams {quantity: 1_000_000.0};
pub const STORAGE_PER_STORAGE_MACHINE: Grams = Grams {quantity: 10_000_000.0};
const STORAGE_PER_WALL: Grams = Grams {quantity: 100_000_000.0};

#[derive(PartialEq)]
pub enum Replacement {
    SplitNetwork,
    Regular,
    Forbidden,
    None,
}

pub struct NetworkUpdate {
    pub air_cleaned: f64,
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
                air_cleaners as f64 * *AIR_CLEANED_PER_CLEANER_PER_UPDATE
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
        power.format()
    }

    fn get_power_generated(&self) -> Watts {
        let solar_panels_count = self.count_tiles_of_type_in(&[TileType::MachineSolarPanel]);
        let power = solar_panels_count as f64 * f64::from(POWER_PER_SOLAR_PANEL);
        power.into()
    }

    pub fn get_power_required_str(&self) -> String {
        let power = self.get_power_required();
        power.format()
    }

    fn get_power_required(&self) -> Watts {
        let machines_count = self.count_tiles_of_type_in(&[
            TileType::MachineDrill,
            TileType::MachineAssembler,
            TileType::MachineAirCleaner,
        ]);
        let power = machines_count as f64 * f64::from(POWER_CONSUMED_PER_MACHINE);
        power.into()
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
            let should_remove = !is_networkable(new_machine);
            if should_remove {
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
    }

    pub fn join(&mut self, other: Network) {
        for node in other.nodes {
            self.add(node);
        }
    }
}

impl Default for Network {
    fn default() -> Self {
        Self::new()
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
