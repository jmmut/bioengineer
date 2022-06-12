use crate::map::{Cell, CellIndex, TileType};
use crate::GameState;
use std::collections::HashSet;

#[derive(Eq, PartialEq, Debug, Copy, Clone)]
pub struct Transformation {
    new_tile_type: TileType,
}

pub fn allowed_transformations(
    cells: &HashSet<CellIndex>,
    game_state: &GameState,
) -> Vec<Transformation> {
    let mut allowed = Vec::new();
    for cell_index in cells.iter() {
        let cell = game_state.map.get_cell(*cell_index);
        allowed.push(allowed_transformations_of_cell(cell));
    }
    let common = set_intersection(allowed);
    common
}

pub fn allowed_transformations_of_cell(cell: &Cell) -> Vec<Transformation> {
    let new_tiles = match cell.tile_type {
        TileType::Unset => {
            panic!("can not transform an UNSET cell!")
        }
        TileType::WallRock => vec![],
        TileType::WallDirt => vec![],
        TileType::FloorRock => vec![TileType::MachineDrill],
        TileType::FloorDirt => vec![],
        TileType::Air => vec![],
        TileType::MachineAssembler => vec![],
        TileType::MachineDrill => vec![],
        TileType::MachineSolarPanel => vec![],
        TileType::MachineShip => vec![],
        TileType::DirtyWaterSurface => vec![],
        TileType::CleanWaterSurface => vec![],
        TileType::DirtyWaterWall => vec![],
        TileType::CleanWaterWall => vec![],
    };
    new_tiles.iter().map(|tile| Transformation::to(*tile)).collect()
}

pub fn set_intersection<T: PartialEq + Copy>(transformations_per_cell: Vec<Vec<T>>) -> Vec<T> {
    if transformations_per_cell.len() == 0 {
        Vec::new()
    } else {
        let mut result = Vec::new();
        for should_be_present_in_all in transformations_per_cell.first().unwrap() {
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

impl Transformation {
    pub fn to(new_tile_type: TileType) -> Self {
        Transformation { new_tile_type }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::map::{Cell, TileType};
    use crate::map::TileType::MachineDrill;

    #[test]
    fn test_basic_transformation() {
        let cell = Cell {
            tile_type: TileType::FloorRock,
        };
        let transformation = allowed_transformations_of_cell(&cell);
        assert_eq!(
            transformation,
            vec![Transformation::to(TileType::MachineDrill)]
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
        let to_drill = Transformation::to(MachineDrill);
        let a = vec![to_drill];
        let b = vec![to_drill];
        let in_both = set_intersection(vec![a, b]);
        assert_eq!(in_both, vec![to_drill]);
    }
}
