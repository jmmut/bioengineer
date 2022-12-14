use crate::world::map::{cell::is_liquid, Cell, CellIndex, Map, TileType};
use std::collections::HashSet;

const AIR_LEVELS_FOR_ALLOWING_SOLAR: i32 = 20;

#[derive(Eq, PartialEq, Debug, Copy, Clone)]
pub struct Transformation {
    pub new_tile_type: TileType,
}

pub fn allowed_transformations(cells: &HashSet<CellIndex>, map: &Map) -> Vec<Transformation> {
    let mut allowed = Vec::new();
    let mut distinct_tiles = HashSet::new();
    for cell_index in cells {
        let cell = map.get_cell(*cell_index);
        distinct_tiles.insert(cell.tile_type as i32);
        allowed.push(allowed_transformations_of_cell(cell, cell_index, map));
    }
    let mut common = set_intersection(allowed);
    common = remove_identity_if_only_one_type(&distinct_tiles, common);
    common
}

fn remove_identity_if_only_one_type(
    distinct_tiles: &HashSet<i32>,
    common: Vec<Transformation>,
) -> Vec<Transformation> {
    if distinct_tiles.len() == 1 {
        let only_tile_selected = distinct_tiles.iter().next().unwrap();
        return common
            .iter()
            .filter(|t| t.new_tile_type as i32 != *only_tile_selected)
            .cloned()
            .collect();
    }
    return common;
}

pub fn allowed_transformations_of_cell(
    cell: &Cell,
    cell_index: &CellIndex,
    map: &Map,
) -> Vec<Transformation> {
    use TileType::*;

    let mut machines = vec![
        // MachineAssembler,
        MachineAirCleaner,
        // MachineDrill,
        Wire,
    ];
    if solar_allowed(cell_index, map) {
        machines.push(MachineSolarPanel);
    }
    let mut new_tiles = match cell.tile_type {
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
            machines.append(&mut vec![Stairs, FloorRock]);
            machines
        }
        Stairs => vec![FloorRock],
        Air => vec![
        // DirtyWaterSurface, DirtyWaterWall, WallRock
        ],
        Wire => vec![FloorRock],
        MachineAssembler => vec![FloorRock],
        MachineAirCleaner => vec![FloorRock],
        MachineDrill => vec![FloorRock],
        MachineSolarPanel => vec![FloorRock],
        MachineShip => vec![FloorRock],
        DirtyWaterSurface => vec![
            // Air
        ],
        CleanWaterSurface => vec![],
        DirtyWaterWall => vec![
            // Air
        ],
        CleanWaterWall => vec![],
    };
    new_tiles.push(cell.tile_type);
    new_tiles
        .iter()
        .map(|tile| Transformation::to(*tile))
        .collect()
}

fn solar_allowed(cell_index: &CellIndex, map: &Map) -> bool {
    above_is(
        TileType::Air,
        AIR_LEVELS_FOR_ALLOWING_SOLAR,
        *cell_index,
        map,
    )
}

fn above_is(expected_tile: TileType, levels: i32, mut cell_index: CellIndex, map: &Map) -> bool {
    let max_height = Map::default_max_cell().y;
    for _ in 0..levels {
        cell_index.y += 1;
        if cell_index.y > max_height {
            return true;
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
                for transformations in transformations_per_cell.iter().skip(1) {
                    if !transformations.contains(should_be_present_in_all) {
                        present = false;
                        break;
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
        if cell.tile_type == TileType::Air {
            if self.new_tile_type == TileType::DirtyWaterWall {
                cell.pressure = 20;
            } else if self.new_tile_type == TileType::DirtyWaterSurface {
                cell.pressure = 10;
            }
        } else if is_liquid(cell.tile_type) {
            if self.new_tile_type == TileType::Air {
                cell.pressure = 0;
            }
        }
        cell.tile_type = self.new_tile_type;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::world::map::{Cell, TileType};

    struct CellTransformationFixture {
        pub cell: Cell,
        // pub min_cell: CellIndex,
        // pub max_cell: CellIndex,
        pub map: Map,
    }

    impl CellTransformationFixture {
        pub fn new() -> Self {
            let cell = Cell::new(TileType::FloorRock);
            let min_cell = CellIndex::new(0, 0, 0);
            let max_cell = CellIndex::new(0, 25, 0);
            let mut map = Map::new_for_cube(min_cell, max_cell);
            map.get_cell_mut(CellIndex::new(0, 0, 0)).tile_type = cell.tile_type;
            for i in 1..=AIR_LEVELS_FOR_ALLOWING_SOLAR {
                map.get_cell_mut(CellIndex::new(0, i, 0)).tile_type = TileType::Air;
            }
            CellTransformationFixture {
                cell,
                // min_cell,
                // max_cell,
                map,
            }
        }
    }

    #[test]
    // #[ignore]
    fn test_basic_surface_transformation() {
        let fx = CellTransformationFixture::new();
        let transformation =
            allowed_transformations_of_cell(&fx.cell, &CellIndex::default(), &fx.map);
        assert_eq!(
            transformation,
            vec![
                // Transformation::to(TileType::MachineAssembler),
                Transformation::to(TileType::MachineAirCleaner),
                // Transformation::to(TileType::MachineDrill),
                Transformation::to(TileType::Wire),
                Transformation::to(TileType::MachineSolarPanel),
                Transformation::to(TileType::Stairs),
                Transformation::to(TileType::FloorRock),
            ]
        );
    }

    #[test]
    fn test_basic_covered_surface_transformation() {
        let mut fx = CellTransformationFixture::new();
        fx.map.get_cell_mut(CellIndex::new(0, 5, 0)).tile_type = TileType::WallRock;

        let transformation =
            allowed_transformations_of_cell(&fx.cell, &CellIndex::default(), &fx.map);
        assert_eq!(
            transformation,
            vec![
                // Transformation::to(TileType::MachineAssembler),
                Transformation::to(TileType::MachineAirCleaner),
                // Transformation::to(TileType::MachineDrill),
                Transformation::to(TileType::Wire),
                Transformation::to(TileType::Stairs),
                Transformation::to(TileType::FloorRock),
            ]
        );
    }

    #[test]
    fn test_no_identity_on_homogeneous_set() {
        let mut fx = CellTransformationFixture::new();
        let cell_indexes = [CellIndex::new(0, 5, 0), CellIndex::new(0, 6, 0)];
        for index in cell_indexes {
            fx.map.get_cell_mut(index).tile_type = TileType::WallRock;
        }
        let transformations = allowed_transformations(&HashSet::from(cell_indexes), &fx.map);
        let contains_wall = transformations.contains(&Transformation::to(TileType::WallRock));
        assert_eq!(contains_wall, false);
    }

    #[test]
    fn test_identity_on_hetergeneous_set() {
        let mut fx = CellTransformationFixture::new();
        let cell_indexes = [CellIndex::new(0, 5, 0), CellIndex::new(0, 6, 0)];
        fx.map.get_cell_mut(cell_indexes[0]).tile_type = TileType::FloorRock;
        fx.map.get_cell_mut(cell_indexes[1]).tile_type = TileType::MachineAirCleaner;
        let transformations = allowed_transformations(&HashSet::from(cell_indexes), &fx.map);
        let contains_cleaner =
            transformations.contains(&Transformation::to(TileType::MachineAirCleaner));
        assert_eq!(contains_cleaner, true);
        let contains_floor = transformations.contains(&Transformation::to(TileType::FloorRock));
        assert_eq!(contains_floor, true);
    }

    #[test]
    fn test_dirt_floor_and_machines_can_become_rock_floor() {
        let mut fx = CellTransformationFixture::new();
        let cell_indexes = [CellIndex::new(0, 5, 0), CellIndex::new(0, 6, 0)];
        fx.map.get_cell_mut(cell_indexes[0]).tile_type = TileType::FloorDirt;
        fx.map.get_cell_mut(cell_indexes[1]).tile_type = TileType::MachineAirCleaner;
        let transformations = allowed_transformations(&HashSet::from(cell_indexes), &fx.map);
        let contains_cleaner =
            transformations.contains(&Transformation::to(TileType::MachineAirCleaner));
        assert_eq!(contains_cleaner, true);
        let contains_floor = transformations.contains(&Transformation::to(TileType::FloorRock));
        assert_eq!(contains_floor, true);
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
