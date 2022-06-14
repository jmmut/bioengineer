use crate::map::{Cell, CellIndex, Map, TileType};
use crate::GameState;
use std::collections::HashSet;

const AIR_LEVELS_FOR_ALLOWING_SOLAR: i32 = 20;

#[derive(Eq, PartialEq, Debug, Copy, Clone)]
pub struct Transformation {
    pub new_tile_type: TileType,
}

pub fn allowed_transformations(
    cells: &HashSet<CellIndex>,
    game_state: &GameState,
) -> Vec<Transformation> {
    let mut allowed = Vec::new();
    for cell_index in cells {
        let cell = game_state.map.get_cell(*cell_index);
        allowed.push(allowed_transformations_of_cell(
            cell,
            cell_index,
            &game_state.map,
        ));
    }
    let common = set_intersection(allowed);
    common
}

pub fn allowed_transformations_of_cell(
    cell: &Cell,
    cell_index: &CellIndex,
    map: &Map,
) -> Vec<Transformation> {
    use TileType::*;

    let machines_with_solar = vec![MachineAssembler, MachineDrill, MachineSolarPanel];
    let machines_without_solar = vec![MachineAssembler, MachineDrill];
    let mut machines = if solar_allowed(cell_index, map) {
        machines_with_solar
    } else {
        machines_without_solar
    };
    let new_tiles = match cell.tile_type {
        Unset => {
            panic!("can not transform an UNSET cell!")
        }
        WallRock => vec![FloorRock, Stairs],
        WallDirt => vec![FloorDirt, Stairs],
        FloorRock => {
            machines.append(&mut vec![Stairs]);
            machines
        }
        FloorDirt => {
            machines.append(&mut vec![Stairs]);
            machines
        }
        Stairs => vec![FloorRock],
        Air => vec![],
        MachineAssembler => vec![FloorRock],
        MachineDrill => vec![FloorRock],
        MachineSolarPanel => vec![FloorRock],
        MachineShip => vec![FloorRock],
        DirtyWaterSurface => vec![],
        CleanWaterSurface => vec![],
        DirtyWaterWall => vec![],
        CleanWaterWall => vec![],
        Robot => vec![],
    };
    new_tiles
        .iter()
        .map(|tile| Transformation::to(*tile))
        .collect()
}

fn solar_allowed(cell_index: &CellIndex, map: &Map) -> bool {
    above_is(TileType::Air, AIR_LEVELS_FOR_ALLOWING_SOLAR, *cell_index, map)
}

fn above_is(expected_tile: TileType, levels: i32, mut cell_index: CellIndex, map: &Map) -> bool {
    let max_height = Map::max_cell().y;
    for _ in 0..levels {
        cell_index.y += 1;
        if cell_index.y > max_height {
            return true
        }
        if map.get_cell(cell_index).tile_type != expected_tile {
            return false;
        }
    }
    true
}

pub fn set_intersection<T: PartialEq + Copy>(transformations_per_cell: Vec<Vec<T>>) -> Vec<T> {
    match transformations_per_cell.first() {
        None => Vec::new(),
        Some(first) => {
            let mut result = Vec::new();
            for should_be_present_in_all in first {
                let mut present = true;
                for i in 1..transformations_per_cell.len() {
                    let transformations = &transformations_per_cell[i];
                    if !transformations.contains(should_be_present_in_all) {
                        present = false;
                    }
                }
                if present {
                    result.push(*should_be_present_in_all);
                }
            }
            result
        }
    }
}

impl Transformation {
    pub fn to(new_tile_type: TileType) -> Self {
        Transformation { new_tile_type }
    }

    pub fn apply(&self, cell: &mut Cell) {
        cell.tile_type = self.new_tile_type;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::map::{Cell, TileType};

    #[test]
    fn test_basic_transformation() {
        let cell = Cell {
            tile_type: TileType::FloorRock,
        };
        let min_cell = CellIndex::new(0, 0, 0);
        let max_cell = CellIndex::new(0, 25, 0);

        let mut map = Map::new_for_cube(min_cell, max_cell);
        map._get_cell_mut(CellIndex::new(0, 0, 0)).tile_type = cell.tile_type;
        for i in 1..=AIR_LEVELS_FOR_ALLOWING_SOLAR {
            map._get_cell_mut(CellIndex::new(0, i, 0)).tile_type = TileType::Air;
        }
        let transformation = allowed_transformations_of_cell(&cell, &CellIndex::default(), &map);
        assert_eq!(
            transformation,
            vec![
                Transformation::to(TileType::MachineAssembler),
                Transformation::to(TileType::MachineDrill),
                Transformation::to(TileType::MachineSolarPanel),
                Transformation::to(TileType::Stairs),
            ]
        );
        map._get_cell_mut(CellIndex::new(0, 5, 0)).tile_type = TileType::WallRock;

        let transformation = allowed_transformations_of_cell(&cell, &CellIndex::default(), &map);
        assert_eq!(
            transformation,
            vec![
                Transformation::to(TileType::MachineAssembler),
                Transformation::to(TileType::MachineDrill),
                Transformation::to(TileType::Stairs),
            ]
        );
    }

    #[test]
    fn test_intersection() {
        let a = vec![1, 2, 3, 6, 7];
        let b = vec![4, 3, 5, 6, 8];
        let in_both = set_intersection(vec![a, b]);
        assert_eq!(in_both, vec![3, 6]);
    }

    #[test]
    fn test_intersection_transformation() {
        let to_drill = Transformation::to(TileType::MachineDrill);
        let a = vec![to_drill];
        let b = vec![to_drill];
        let in_both = set_intersection(vec![a, b]);
        assert_eq!(in_both, vec![to_drill]);
    }
}
