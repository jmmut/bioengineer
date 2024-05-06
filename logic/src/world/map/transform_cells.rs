use crate::world::map::cell::DEFAULT_HEALTH;
use crate::world::map::{cell::is_liquid, Cell, CellIndex, Map, TileType};
use crate::world::networks::network::{Addition, Replacement};
use crate::world::robots::{DOWN, UP};
use std::collections::HashSet;

const AIR_LEVELS_FOR_ALLOWING_SOLAR: i32 = 20;

#[derive(Eq, PartialEq, Debug, Copy, Clone)]
pub struct Transformation {
    pub new_tile_type: TileType,
}

#[derive(PartialEq, Eq, Hash, Copy, Clone)]
pub enum TransformationFailure {
    NotEnoughMaterial,
    NotEnoughStorage,
    AboveWouldCollapse,
    NoSturdyBase,
    WouldOccludeSolarPanel,
    OutOfShipReach,
    CanNotDeconstructShip,
    SplitNetwork,
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
        MachineStorage,
    ];
    if solar_allowed(cell_index, map) {
        machines.push(MachineSolarPanel);
    }
    let machines_plus = |mut tiles: Vec<TileType>| -> Vec<TileType> {
        machines.append(&mut tiles);
        machines
    };
    let mut new_tiles = match cell.tile_type {
        Unset => {
            panic!("can not transform an UNSET cell!")
        }
        WallRock => machines_plus(vec![TreeHealthy]),
        WallDirt => machines_plus(vec![TreeHealthy]),
        FloorRock => machines_plus(vec![TreeHealthy]),
        FloorDirt => machines_plus(vec![FloorRock, TreeHealthy]),
        Stairs => machines_plus(vec![FloorRock, TreeHealthy]),
        Air => machines_plus(vec![
            // DirtyWaterSurface, DirtyWaterWall,
            WallRock, // FloorRock,
            // WallDirt,
            // FloorDirt,
            TreeHealthy,
        ]),
        Wire | MachineAssembler | MachineAirCleaner | MachineDrill | MachineSolarPanel
        | MachineStorage => machines_plus(vec![WallRock, TreeHealthy]),
        MachineShip => machines_plus(vec![WallRock, TreeHealthy]),
        // DirtyWaterSurface => vec![
        //     WallRock, FloorRock,
        //     // Air
        // ],
        // CleanWaterSurface => vec![WallRock, FloorRock],
        // DirtyWaterWall => vec![
        //     WallRock, FloorRock,
        //     // Air
        // ],
        // CleanWaterWall => vec![WallRock, FloorRock],
        TreeHealthy => machines_plus(vec![WallRock]),
        TreeSparse => machines_plus(vec![WallRock]),
        TreeDying => machines_plus(vec![WallRock]),
        TreeDead => machines_plus(vec![WallRock]),
    };
    new_tiles.push(cell.tile_type);
    if cell.tile_type != Air && cell.tile_type != MachineShip {
        new_tiles.push(Air);
    }
    new_tiles
        .iter()
        .map(|tile| Transformation::to(*tile))
        .collect()
}

fn solar_allowed(cell_index: &CellIndex, map: &Map) -> bool {
    column_above_is(
        TileType::Air,
        AIR_LEVELS_FOR_ALLOWING_SOLAR,
        *cell_index,
        map,
    )
}

pub fn column_above_is(
    expected_tile: TileType,
    levels: i32,
    mut cell_index: CellIndex,
    map: &Map,
) -> bool {
    let max_height = map.max_cell.y;
    for _ in 0..levels {
        cell_index.y += 1;
        if cell_index.y > max_height {
            println!("Bug: peeking above the map limits");
            return true;
        }
        let actual_tile = map.get_cell(cell_index).tile_type;
        if actual_tile != expected_tile {
            return false;
        }
    }
    true
}

pub fn above_is(expected_tile: TileType, cell_index: CellIndex, map: &Map) -> bool {
    column_above_is(expected_tile, 1, cell_index, map)
}

pub fn above(predicate: impl Fn(TileType) -> bool, cell_index: CellIndex, map: &Map) -> bool {
    let neighbour = cell_index + UP;
    return if neighbour.y > map.max_cell.y {
        println!("Bug: peeking above the map limits");
        true
    } else {
        predicate(map.get_cell(neighbour).tile_type)
    };
}

pub fn below_is(expected_tile: TileType, cell_index: CellIndex, map: &Map) -> bool {
    below(|tile| tile == expected_tile, cell_index, map)
}

pub fn below(predicate: impl Fn(TileType) -> bool, cell_index: CellIndex, map: &Map) -> bool {
    let neighbour = cell_index + DOWN;
    return if neighbour.y < map.min_cell.y {
        println!("Bug: peeking below the map limits");
        true
    } else {
        predicate(map.get_cell(neighbour).tile_type)
    };
}

pub fn set_intersection<T: Eq + Copy>(transformations_per_cell: Vec<Vec<T>>) -> Vec<T> {
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
            let mut reversed_result = result.iter().rev().collect::<Vec<_>>();
            let mut unique = Vec::<T>::new();
            while let Some(last) = reversed_result.pop() {
                if !unique.contains(&last) {
                    unique.push(*last);
                }
            }
            unique
        }
    }
}

impl Transformation {
    pub fn to(new_tile_type: TileType) -> Self {
        Transformation { new_tile_type }
    }

    pub fn apply(&self, cell: &mut Cell) {
        if is_liquid(cell.tile_type) && self.new_tile_type == TileType::Air
            || self.new_tile_type == TileType::WallRock
        {
            cell.pressure = 0;
            cell.renderable_pressure = 0;
        }
        if self.new_tile_type == TileType::TreeHealthy {
            cell.health = DEFAULT_HEALTH;
        }
        cell.tile_type = self.new_tile_type;
    }
}

impl From<Addition> for Option<TransformationFailure> {
    fn from(addition: Addition) -> Self {
        match addition {
            Addition::Ok => None,
            Addition::NotEnoughMaterial => Some(TransformationFailure::NotEnoughMaterial),
            Addition::NotEnoughStorage => Some(TransformationFailure::NotEnoughStorage),
        }
    }
}
impl From<Replacement> for Option<TransformationFailure> {
    fn from(replacement: Replacement) -> Self {
        match replacement {
            Replacement::Ok | Replacement::None => None,
            Replacement::NotEnoughMaterial => Some(TransformationFailure::NotEnoughMaterial),
            Replacement::NotEnoughStorage => Some(TransformationFailure::NotEnoughStorage),
            Replacement::Forbidden => Some(TransformationFailure::CanNotDeconstructShip),
            Replacement::SplitNetwork => Some(TransformationFailure::SplitNetwork),
        }
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
    #[ignore]
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
                Transformation::to(TileType::TreeHealthy),
                Transformation::to(TileType::FloorRock),
            ]
        );
    }

    #[test]
    #[ignore]
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
                Transformation::to(TileType::TreeHealthy),
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
        fx.map.get_cell_mut(cell_indexes[0]).tile_type = TileType::WallRock;
        fx.map.get_cell_mut(cell_indexes[1]).tile_type = TileType::MachineAirCleaner;
        let transformations = allowed_transformations(&HashSet::from(cell_indexes), &fx.map);
        let contains_cleaner =
            transformations.contains(&Transformation::to(TileType::MachineAirCleaner));
        assert_eq!(contains_cleaner, true);
        let contains_floor = transformations.contains(&Transformation::to(TileType::WallRock));
        assert_eq!(contains_floor, true);
    }

    #[test]
    fn test_air_and_machines_can_become_rock_wall() {
        let mut fx = CellTransformationFixture::new();
        let cell_indexes = [CellIndex::new(0, 5, 0), CellIndex::new(0, 6, 0)];
        fx.map.get_cell_mut(cell_indexes[0]).tile_type = TileType::Air;
        fx.map.get_cell_mut(cell_indexes[1]).tile_type = TileType::MachineAirCleaner;
        let transformations = allowed_transformations(&HashSet::from(cell_indexes), &fx.map);
        let contains_cleaner =
            transformations.contains(&Transformation::to(TileType::MachineAirCleaner));
        assert_eq!(contains_cleaner, true);
        let contains_floor = transformations.contains(&Transformation::to(TileType::WallRock));
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

    #[test]
    fn test_intersection_duplicates() {
        let a = vec![1, 2, 3, 2];
        let b = vec![3, 2, 8];
        let in_both = set_intersection(vec![a, b]);
        assert_eq!(in_both, vec![2, 3]);
    }
}
