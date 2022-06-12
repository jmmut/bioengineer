use std::collections::HashSet;
use crate::GameState;
use crate::map::{Cell, CellIndex, TileType};

#[derive(Eq, PartialEq, Debug)]
pub struct Transformation {
    new_tile_type: TileType,
}


pub fn allowed_transformations(cells: &HashSet<CellIndex>, game_state: &GameState) -> Vec<Transformation> {
    let mut allowed = Vec::new();
    for cell_index in cells.iter() {
        let cell = game_state.map.get_cell(*cell_index);
        allowed.push(allowed_transformations_of_cell(cell));
    }
    let common = set_intersection(allowed);
    common
}

pub fn allowed_transformations_of_cell(cell: &Cell) -> Vec<Transformation> {
    Vec::new()
}

pub fn set_intersection(transformations_per_cell: Vec<Vec<Transformation>>) -> Vec<Transformation> {
    Vec::new()
}

impl Transformation {
    pub fn to(new_tile_type: TileType) -> Self {
        Transformation {new_tile_type}
    }
}

#[cfg(test)]
mod tests {
    use crate::map::{Cell, TileType};
    use super::*;

    #[test]
    fn test_basic_transformation() {
        let cell = Cell { tile_type: TileType::FloorRock };
        let transformation = allowed_transformations_of_cell(&cell);
        assert_eq!(transformation, vec![Transformation::to(TileType::MachineDrill)]);
    }
}
