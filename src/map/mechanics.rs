use crate::map::{Cell, CellIndex, TileType};
use crate::GameState;
use std::collections::HashSet;

#[derive(Eq, PartialEq, Debug)]
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

pub fn set_intersection(transformations_per_cell: Vec<Vec<Transformation>>) -> Vec<Transformation> {
    Vec::new()
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
}
