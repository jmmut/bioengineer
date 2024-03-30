pub mod air;
pub mod change_tiles;
pub mod floodable;

use mq_basics::IVec3;
use super::*;
use crate::world::map::{Map, PressureAndType};

fn assert_steps_2x2(maps: Vec<Vec<i32>>) {
    let min_cell = CellIndex::new(0, 0, 0);
    let max_cell = CellIndex::new(2, 0, 2);
    assert_steps(maps, min_cell, max_cell)
}

fn assert_steps(maps: Vec<Vec<Pressure>>, min_cell: CellIndex, max_cell: CellIndex) {
    assert!(maps.len() >= 2);
    for i in 1..maps.len() {
        let initial = maps[i - 1].clone();
        let expected = maps[i].clone();
        let mut map = Map::_new_from_pressures(initial, min_cell, max_cell);
        advance_fluid(&mut map);
        let computed = map._get_pressures(min_cell, max_cell);
        assert_eq!(computed, expected);
    }
}

fn assert_steps_with_types(
    maps: Vec<Vec<PressureAndType>>,
    min_cell: CellIndex,
    max_cell: CellIndex,
) {
    assert!(maps.len() >= 2);
    for i in 1..maps.len() {
        let initial = maps[i - 1].clone();
        let expected = maps[i].clone();
        let mut map = Map::_new_from_pressures_and_tiles(initial, min_cell, max_cell);
        advance_fluid(&mut map);
        let computed = map._get_pressures_and_types(min_cell, max_cell);
        assert_eq!(computed, expected);
    }
}

fn assert_n_steps(
    initial_map: Vec<Pressure>,
    final_map: Vec<Pressure>,
    iterations: i32,
    min_cell: CellIndex,
    max_cell: CellIndex,
) -> Vec<Pressure> {
    let mut map = Map::_new_from_pressures(initial_map, min_cell, max_cell);
    for _ in 0..iterations {
        advance_fluid(&mut map);
    }
    let computed = map._get_pressures(min_cell, max_cell);
    assert_eq!(computed, final_map);
    computed
}

fn compute_n_steps(
    min_cell: IVec3,
    max_cell: IVec3,
    cells: &[i32],
    iterations: i32,
    mode: FluidMode,
) -> Vec<i32> {
    let mut map = Map::_new_from_pressures(cells.to_vec(), min_cell, max_cell);
    let mut fluids = Fluids::new(mode);
    for _ in 0..iterations {
        fluids.advance(&mut map);
    }
    map._get_pressures(min_cell, max_cell)
}
