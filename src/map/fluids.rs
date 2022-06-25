use crate::map::TileType::Air;
use crate::map::{
    is_liquid, is_liquid_or_air, CellCubeIterator, CellIndex, Map, Pressure, TileType,
};

const VERTICAL_PRESSURE_DIFFERENCE: i32 = 10;

pub fn advance_fluid(map: &mut Map) {
    let min_cell = map.min_cell();
    let max_cell = map.max_cell();
    let is_valid = |cell_index: CellIndex, map: &Map| {
        cell_index.x >= min_cell.x
            && cell_index.x <= max_cell.x
            && cell_index.y >= min_cell.y
            && cell_index.y <= max_cell.y
            && cell_index.z >= min_cell.z
            && cell_index.z <= max_cell.z
            && is_liquid_or_air(map.get_cell(cell_index).tile_type)
    };
    let yp = CellIndex::new(0, 1, 0);
    let yn = CellIndex::new(0, -1, 0);

    let iter = CellCubeIterator::new(min_cell, max_cell);
    for cell_index in iter {
        let cell = map.get_cell(cell_index);
        if is_liquid(cell.tile_type) {
            let current_pressure = cell.pressure;
            let next_pressure = cell.next_pressure;
            let mut flow = Vec::new();
            let dir = yn;
            if is_valid(cell_index + dir, map) {
                let adjacent_cell = map.get_cell(cell_index + dir);
                if adjacent_cell.pressure
                    < (current_pressure + next_pressure + VERTICAL_PRESSURE_DIFFERENCE)
                {
                    flow.push(dir);
                }
                //+1: in the other directions we don't want to keep at least 1 pressure,
                // but we don't want to keep 1 pressure in this cell if it can go below
                prepare_next_pressure(map, cell_index, current_pressure, next_pressure + 1, flow);
            }
        }
    }
    let xp = CellIndex::new(1, 0, 0);
    let xn = CellIndex::new(-1, 0, 0);
    let zp = CellIndex::new(0, 0, 1);
    let zn = CellIndex::new(0, 0, -1);
    let iter = CellCubeIterator::new(min_cell, max_cell);
    for cell_index in iter {
        let cell = map.get_cell(cell_index);
        if is_liquid(cell.tile_type) {
            let current_pressure = cell.pressure;
            let next_pressure = cell.next_pressure;
            let mut flow = Vec::new();
            let mut add_flow_direction = |dir: CellIndex, map: &Map| {
                if is_valid(cell_index + dir, map) {
                    let adjacent_cell = map.get_cell(cell_index + dir);
                    if adjacent_cell.pressure < current_pressure + next_pressure {
                        flow.push(dir);
                    }
                }
            };
            add_flow_direction(xp, map);
            add_flow_direction(xn, map);
            add_flow_direction(zp, map);
            add_flow_direction(zn, map);
            prepare_next_pressure(map, cell_index, current_pressure, next_pressure, flow);
        }
    }
    let iter = CellCubeIterator::new(min_cell, max_cell);
    for cell_index in iter {
        let cell = map.get_cell(cell_index);
        if is_liquid(cell.tile_type) {
            let current_pressure = cell.pressure;
            let next_pressure = cell.next_pressure;
            let mut flow = Vec::new();
            let dir = yp;
            if is_valid(cell_index + dir, map) {
                let adjacent_cell = map.get_cell(cell_index + dir);
                if adjacent_cell.pressure
                    < (current_pressure + next_pressure - VERTICAL_PRESSURE_DIFFERENCE)
                {
                    flow.push(dir);
                }
            }
            prepare_next_pressure(map, cell_index, current_pressure, next_pressure, flow);
        }
    }

    swap_next_pressure_to_current(map, min_cell, max_cell)
}

fn prepare_next_pressure(
    map: &mut Map,
    cell_index: CellIndex,
    current_pressure: Pressure,
    next_pressure: Pressure,
    flow: Vec<CellIndex>,
) {
    if current_pressure + next_pressure > flow.len() as i32 {
        map.get_cell_mut(cell_index).next_pressure -= flow.len() as i32;
        for dir in flow {
            map.get_cell_mut(cell_index + dir).next_pressure += 1;
        }
    }
}

fn swap_next_pressure_to_current(map: &mut Map, min_cell: CellIndex, max_cell: CellIndex) {
    let iter = CellCubeIterator::new(min_cell, max_cell);
    for cell_index in iter {
        let something_above = {
            let above_cell = map.get_cell_mut(cell_index + CellIndex::new(0, 1, 0));
            ((above_cell.pressure + above_cell.next_pressure) > 0) || above_cell.tile_type != Air
        };
        let cell = map.get_cell_mut(cell_index);
        if is_liquid_or_air(cell.tile_type) {
            if cell.pressure + cell.next_pressure < 0 {
                panic!(
                    "negative pressure! for cell {}, with pressure {}, next pressure {}.",
                    cell_index, cell.pressure, cell.next_pressure
                );
            }
            cell.pressure += cell.next_pressure;
            cell.tile_type = if cell.pressure <= 0 {
                TileType::Air
            } else if something_above {
                TileType::DirtyWaterWall
            } else {
                // if pressure_above > 0 {
                //     println!("above cell should be air!");
                // }
                TileType::DirtyWaterSurface
            };
            cell.next_pressure = 0;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::map::Map;
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
    ) {
        let mut map = Map::_new_from_pressures(initial_map, min_cell, max_cell);
        for _ in 0..iterations {
            advance_fluid(&mut map);
        }
        let computed = map._get_pressures(min_cell, max_cell);
        assert_eq!(computed, final_map);
    }

    #[test]
    fn test_basic_fluid() {
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
    fn test_basic_3d() {
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
        let result_horizontal = panic::catch_unwind(|| {
            assert_steps_2x2(vec![cells, expected]);
        });

        let cells = vec![11, 0];
        let expected = vec![11, 0];
        let result_upwards = panic::catch_unwind(|| {
            assert_steps(
                vec![cells, expected],
                CellIndex::new(0, 0, 0),
                CellIndex::new(0, 1, 0),
            );
        });

        let cells = vec![10, 1];
        let expected = vec![10, 1];
        let result_downwards = panic::catch_unwind(|| {
            assert_steps(
                vec![cells, expected],
                CellIndex::new(0, 0, 0),
                CellIndex::new(0, 1, 0),
            );
        });

        if result_upwards.is_ok() && result_downwards.is_ok() && result_horizontal.is_ok() {
            // ok, all directions are stable
        } else if result_upwards.is_err() && result_downwards.is_err() && result_horizontal.is_err()
        {
            // ok, all directions are dynamic
        } else {
            assert!(
                false,
                "All directions should be stable or dynamic. \
                upwards stable? {}, downwards stable? {}, horizontal stable? {}",
                result_upwards.is_ok(),
                result_downwards.is_ok(),
                result_horizontal.is_ok()
            )
        }
    }

    #[test]
    fn test_many_iterations_3d() {
        let min_cell = CellIndex::new(0, 0, 0);
        let max_cell = CellIndex::new(0, 2, 2);
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
        assert_n_steps(cells.clone(), expected, 1, min_cell, max_cell);

        #[rustfmt::skip]
        let expected = vec![
            50, 1, 1,
            50, -1, 0,
            48, -1, 0,
        ];
        assert_n_steps(cells.clone(), expected, 2, min_cell, max_cell);

        #[rustfmt::skip]
        let expected = vec![
            50, 2, 1,
            50, -1, 0,
            47, -1, 0,
        ];
        assert_n_steps(cells.clone(), expected, 3, min_cell, max_cell);

        #[rustfmt::skip]
        let expected = vec![
            50, 2, 2,
            50, -1, 0,
            46, -1, 0,
        ];
        assert_n_steps(cells.clone(), expected, 4, min_cell, max_cell);

        #[rustfmt::skip]
        let expected = vec![
            50, 10, 10,
            45, -1, 0,
            35, -1, 0,
        ];
        assert_n_steps(cells.clone(), expected, 20, min_cell, max_cell);

        #[rustfmt::skip]
        let expected = vec![
            50, 11, 10,
            44, -1, 1,
            34, -1, 0,
        ];
        assert_n_steps(cells.clone(), expected, 22, min_cell, max_cell);

        #[rustfmt::skip]
        let expected = vec![
            50, 12, 11,
            43, -1, 0,
            34, -1, 0,
        ];
        assert_n_steps(cells.clone(), expected, 23, min_cell, max_cell);

        #[rustfmt::skip]
        let expected = vec![
            31, 30, 30,
            20, -1, 19,
            11, -1, 9,
        ];
        assert_n_steps(cells.clone(), expected, 90, min_cell, max_cell);

        #[rustfmt::skip]
        let expected = vec![
            30, 31, 29,
            21, -1, 19,
            10, -1, 10,
        ];
        assert_n_steps(cells.clone(), expected, 91, min_cell, max_cell);

        #[rustfmt::skip]
        let expected = vec![
            31, 29, 29,
            20, -1, 20,
            11, -1, 10,
        ];
        assert_n_steps(cells.clone(), expected, 92, min_cell, max_cell);
    }
}
