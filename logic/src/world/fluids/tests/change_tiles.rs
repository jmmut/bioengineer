use super::*;
use crate::world::map::TileType;

#[test]
fn test_emptying() {
    let cells = vec![5, 1];
    let max_cell = CellIndex::new(0, 1, 0);
    let min_cell = CellIndex::new(0, 0, 0);
    let mut map = Map::_new_from_pressures(cells, min_cell, max_cell);
    advance_fluid(&mut map);
    assert_eq!(map.get_cell(min_cell).pressure, 6);
    assert_eq!(map.get_cell(min_cell).tile_type, TileType::Air);
    assert_eq!(map.get_cell(max_cell).pressure, 0);
    assert_eq!(map.get_cell(max_cell).tile_type, TileType::Air);
}

#[test]
fn test_filling() {
    let cells = vec![20, 0, 0];
    let max_cell = CellIndex::new(0, 2, 0);
    let middle_cell = CellIndex::new(0, 1, 0);
    let min_cell = CellIndex::new(0, 0, 0);
    let mut map = Map::_new_from_pressures(cells, min_cell, max_cell);
    advance_fluid(&mut map);
    assert_eq!(map.get_cell(min_cell).pressure, 19);
    assert_eq!(map.get_cell(min_cell).tile_type, TileType::Air);
    assert_eq!(map.get_cell(middle_cell).pressure, 1);
    assert_eq!(map.get_cell(middle_cell).tile_type, TileType::Air);
    assert_eq!(map.get_cell(max_cell).pressure, 0);
    assert_eq!(map.get_cell(max_cell).tile_type, TileType::Air);
}
