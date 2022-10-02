use super::*;
use crate::world::map::Map;
use crate::IVec3;
use std::panic;

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

#[test]
fn test_basic_2d_top_view() {
    #[rustfmt::skip]
    let cells = vec![
        0, 0, 0,
        0, 5, 0,
        0, 0, 0,
    ];
    #[rustfmt::skip]
    let expected = vec![
        0, 1, 0,
        1, 1, 1,
        0, 1, 0,
    ];
    assert_steps_2x2(vec![cells, expected]);
}

#[test]
fn test_no_movement() {
    #[rustfmt::skip]
    let cells = vec![
        0, 0, 0,
        0, 3, 0,
        0, 0, 0,
    ];
    #[rustfmt::skip]
    let expected = vec![
        0, 0, 0,
        0, 3, 0,
        0, 0, 0,
    ];
    assert_steps_2x2(vec![cells, expected]);
}

#[test]
fn test_2_steps() {
    #[rustfmt::skip]
    let cells = vec![
        0, 0, 0,
        0, 10, 0,
        0, 0, 0,
    ];
    #[rustfmt::skip]
    let expected_1 = vec![
        0, 1, 0,
        1, 6, 1,
        0, 1, 0,
    ];
    #[rustfmt::skip]
    let expected_2 = vec![
        0, 2, 0,
        2, 2, 2,
        0, 2, 0,
    ];
    assert_steps_2x2(vec![cells, expected_1, expected_2]);
}

#[test]
fn test_borders() {
    let cells = vec![10, 0];
    let expected = vec![9, 1];
    assert_steps(
        vec![cells, expected],
        CellIndex::new(0, 0, 0),
        CellIndex::new(0, 0, 1),
    );
}

#[test]
fn test_basic_2d_side_view() {
    #[rustfmt::skip]
    let cells = vec![
        50, 0, 0,
        50, -1, 0,
        50, -1, 0,
    ];
    #[rustfmt::skip]
    let expected = vec![
        50, 1, 0,
        50, -1, 0,
        49, -1, 0,
    ];
    assert_n_steps(
        cells,
        expected,
        1,
        CellIndex::new(0, 0, 0),
        CellIndex::new(0, 2, 2),
    );
}

#[test]
fn test_basic_vertical_upwards() {
    let cells = vec![12, 0];
    let expected = vec![11, 1];
    assert_steps(
        vec![cells, expected.clone(), expected],
        CellIndex::new(0, 0, 0),
        CellIndex::new(0, 1, 0),
    );
}

#[test]
fn test_basic_vertical_downwards() {
    let cells = vec![10, 2];
    let expected = vec![11, 1];
    assert_steps(
        vec![cells, expected.clone(), expected],
        CellIndex::new(0, 0, 0),
        CellIndex::new(0, 1, 0),
    );
}

#[test]
fn test_minimize_movement() {
    #[rustfmt::skip]
    let cells = vec![
        0, 0, 0,
        0, 4, 0,
        0, 0, 0,
    ];
    #[rustfmt::skip]
    let expected = vec![
        0, 0, 0,
        0, 4, 0,
        0, 0, 0,
    ];
    let result_sideways_stable = panic::catch_unwind(|| {
        assert_steps_2x2(vec![cells, expected]);
    })
    .is_ok();

    let cells = vec![11, 0];
    let expected = vec![11, 0];
    let result_upwards_stable = panic::catch_unwind(|| {
        assert_steps(
            vec![cells, expected],
            CellIndex::new(0, 0, 0),
            CellIndex::new(0, 1, 0),
        );
    })
    .is_ok();

    let cells = vec![10, 1];
    let expected = vec![10, 1];
    let result_downwards_stable = panic::catch_unwind(|| {
        assert_steps(
            vec![cells, expected],
            CellIndex::new(0, 0, 0),
            CellIndex::new(0, 1, 0),
        );
    })
    .is_ok();

    assert!(
        !result_downwards_stable,
        "We never want to make downwards flow stable. \
                It would make a column of 1-pressure water."
    );

    assert!(
        result_upwards_stable,
        "We always want to make upwards flow stable. \
                Otherwise it would likely create and remove a water tile above."
    );
    assert!(!result_sideways_stable);
}

#[test]
fn test_many_iterations_2d_side_view() {
    let min_cell = CellIndex::new(0, 0, 0);
    let max_cell = CellIndex::new(0, 2, 2);
    let mut i = 0;
    let mut assert_until =
        |initial_map: Vec<Pressure>, final_map: Vec<Pressure>, iterations: i32| -> Vec<Pressure> {
            let computed =
                assert_n_steps(initial_map, final_map, iterations - i, min_cell, max_cell);
            i = iterations;
            computed
        };

    #[rustfmt::skip]
    let mut cells = vec![
        50, 0, 0,
        50, -1, 0,
        50, -1, 0,
    ];
    #[rustfmt::skip]
    let expected = vec![
        50, 1, 0,
        50, -1, 0,
        49, -1, 0,
    ];
    cells = assert_until(cells, expected, 1);

    #[rustfmt::skip]
    let expected = vec![
        50, 1, 1,
        50, -1, 0,
        48, -1, 0,
    ];
    cells = assert_until(cells, expected, 2);

    #[rustfmt::skip]
    let expected = vec![
        50, 2, 1,
        50, -1, 0,
        47, -1, 0,
    ];
    cells = assert_until(cells, expected, 3);

    #[rustfmt::skip]
    let expected = vec![
        50, 2, 2,
        50, -1, 0,
        46, -1, 0,
    ];
    cells = assert_until(cells, expected, 4);

    #[rustfmt::skip]
    let expected = vec![
        50, 10, 10,
        45, -1, 0,
        35, -1, 0,
    ];
    cells = assert_until(cells, expected, 20);

    #[rustfmt::skip]
    let expected = vec![
        50, 11, 11,
        44, -1, 0,
        34, -1, 0,
    ];
    cells = assert_until(cells, expected, 22);

    #[rustfmt::skip]
    let expected = vec![
        50, 12, 11,
        43, -1, 0,
        34, -1, 0,
    ];
    cells = assert_until(cells, expected, 23);

    #[rustfmt::skip]
    let expected = vec![
        31, 30, 30,
        20, -1, 20,
        11, -1, 8,
    ];
    cells = assert_until(cells, expected, 90);

    #[rustfmt::skip]
    let expected = vec![
        30, 31, 30,
        21, -1, 19,
        10, -1, 9,
    ];
    cells = assert_until(cells, expected, 91);

    #[rustfmt::skip]
    let expected = vec![
        31, 30, 30,
        20, -1, 20,
        10, -1, 9,
    ];
    cells = assert_until(cells, expected, 92);

    #[rustfmt::skip]
    let expected = vec![
        30, 31, 30,
        20, -1, 20,
        10, -1, 9,
    ];
    let final_expected_loop = expected.clone();
    cells = assert_until(cells, expected, 93);

    #[rustfmt::skip]
    let expected = vec![
        31, 29, 31,
        20, -1, 20,
        10, -1, 9,
    ];
    cells = assert_until(cells, expected, 94);

    let _ = assert_until(cells, final_expected_loop, 95);
}

#[test]
fn test_staged_is_identical() {
    let min_cell = CellIndex::new(0, 0, 0);
    let max_cell = CellIndex::new(0, 2, 2);
    #[rustfmt::skip]
    let cells = vec![
        50, 0, 0,
        50, -1, 0,
        50, -1, 0,
    ];
    let iterations = 90;

    let computed_together = compute_n_steps(
        min_cell,
        max_cell,
        &cells,
        iterations,
        FluidMode::AllTogether,
    );
    let computed_in_stages = compute_n_steps(
        min_cell,
        max_cell,
        &cells,
        iterations * 5,
        FluidMode::InStages,
    );

    assert_eq!(computed_in_stages, computed_together);
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

mod change_tiles {
    use super::*;

    #[test]
    fn test_emptying() {
        let cells = vec![5, 1];
        let max_cell = CellIndex::new(0, 1, 0);
        let min_cell = CellIndex::new(0, 0, 0);
        let mut map = Map::_new_from_pressures(cells, min_cell, max_cell);
        advance_fluid(&mut map);
        assert_eq!(map.get_cell(min_cell).pressure, 6);
        assert_eq!(
            map.get_cell(min_cell).tile_type,
            TileType::DirtyWaterSurface
        );
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
        assert_eq!(map.get_cell(min_cell).tile_type, TileType::DirtyWaterWall);
        assert_eq!(map.get_cell(middle_cell).pressure, 1);
        assert_eq!(
            map.get_cell(middle_cell).tile_type,
            TileType::DirtyWaterSurface
        );
        assert_eq!(map.get_cell(max_cell).pressure, 0);
        assert_eq!(map.get_cell(max_cell).tile_type, TileType::Air);
    }
}
