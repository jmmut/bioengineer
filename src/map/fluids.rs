use crate::map::chunk::cell_iter::CellIterItem;
use crate::map::ref_mut_iterator::RefMutIterator;
use crate::map::{
    cell::is_liquid, cell::is_liquid_or_air, cell::Pressure, CellCubeIterator, CellIndex, Map,
    TileType,
};
use crate::IVec3;

const VERTICAL_PRESSURE_DIFFERENCE: i32 = 10;

pub fn advance_fluid(map: &mut Map) {
    #[cfg(test)]
    println!("advancing");
    #[cfg(test)]
    print_map_pressures(map);

    advance_fluid_downwards(map);
    #[cfg(test)]
    print_map_pressures(map);

    advance_fluid_sideways(map);
    #[cfg(test)]
    print_map_pressures(map);

    advance_fluid_upwards(map);
    #[cfg(test)]
    print_map_pressures(map);
    /*
       for (cell_index, cell) in &*map {
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
               // +1: make it dynamic. Otherwise it will be stable (stable means including oneself
               // when giving out pressure)
               prepare_next_pressure(map, cell_index, current_pressure, next_pressure + 1, flow);
           }
       }
       for (cell_index, cell) in &*map {
           if is_liquid(cell.tile_type) {
               let current_pressure = cell.pressure;
               let next_pressure = cell.next_pressure;
               let mut flow = Vec::new();
               let dir = yp;
               if is_valid(cell_index + dir, map) {
                   let adjacent_cell = map.get_cell(cell_index + dir);
                   if adjacent_cell.pressure + 1
                       < (current_pressure + next_pressure - VERTICAL_PRESSURE_DIFFERENCE)
                   {
                       flow.push(dir);
                   }
               }
               prepare_next_pressure(map, cell_index, current_pressure, next_pressure, flow);
           }
       }

       swap_next_pressure_to_current(map, min_cell, max_cell)

    */
}

fn print_map_pressures(map: &Map) {
    let iter = CellCubeIterator::new(map.min_cell(), map.max_cell());
    let mut pressures = Vec::new();
    for cell_index in iter {
        pressures.push(map.get_cell(cell_index).pressure)
    }
    println!("pressures: {:?}", pressures);
}

fn advance_fluid_downwards(map: &mut Map) {
    let yp = CellIndex::new(0, 1, 0);
    let yn = CellIndex::new(0, -1, 0);
    let pressure_threshold = -VERTICAL_PRESSURE_DIFFERENCE;
    let flow = Flow::new(map, pressure_threshold);
    let updated_map = map.clone();
    let mut iter = updated_map.iter_mut();
    while let Option::Some(CellIterItem { cell_index, cell }) = iter.next() {
        if is_liquid(cell.tile_type) {
            let current_pressure = cell.pressure;
            let mut pressure_diff = 0;
            flow.flow_outwards(cell_index + yn, current_pressure, &mut pressure_diff);
            flow.flow_inwards(cell_index + yp, current_pressure, &mut pressure_diff);
            cell.pressure += pressure_diff;
        }
    }
    *map = Map::new_from_iter(iter);
}

fn advance_fluid_sideways(map: &mut Map) {
    let xp = CellIndex::new(1, 0, 0);
    let xn = CellIndex::new(-1, 0, 0);
    let zp = CellIndex::new(0, 0, 1);
    let zn = CellIndex::new(0, 0, -1);
    let pressure_threshold = 0;
    let flow = Flow::new(map, pressure_threshold);
    let updated_map = map.clone();
    let mut iter = updated_map.iter_mut();
    while let Option::Some(CellIterItem { cell_index, cell }) = iter.next() {
        if is_liquid(cell.tile_type) {
            let current_pressure = cell.pressure;
            let mut pressure_diff = 0;
            flow.flow_outwards(cell_index + xp, current_pressure, &mut pressure_diff);
            flow.flow_outwards(cell_index + xn, current_pressure, &mut pressure_diff);
            flow.flow_outwards(cell_index + zp, current_pressure, &mut pressure_diff);
            flow.flow_outwards(cell_index + zn, current_pressure, &mut pressure_diff);
            let next_pressure = pressure_diff + cell.pressure;
            // change this to > for stable, >= for dynamic. see test_minimize_movement()
            cell.can_flow_out = next_pressure >= 0;
            if cell.can_flow_out {
                cell.next_pressure = next_pressure;
            } else {
                cell.next_pressure = cell.pressure
            }
        }
    }
    *map = Map::new_from_iter(iter);
    let updated_map = map.clone();
    let flow = Flow::new(map, pressure_threshold);
    let mut iter = updated_map.iter_mut();
    while let Option::Some(CellIterItem { cell_index, cell }) = iter.next() {
        if is_liquid(cell.tile_type) {
            let current_pressure = cell.pressure;
            let mut pressure_diff = 0;
            flow.maybe_flow_inwards(cell_index + xp, current_pressure, &mut pressure_diff);
            flow.maybe_flow_inwards(cell_index + xn, current_pressure, &mut pressure_diff);
            flow.maybe_flow_inwards(cell_index + zp, current_pressure, &mut pressure_diff);
            flow.maybe_flow_inwards(cell_index + zn, current_pressure, &mut pressure_diff);
            cell.pressure = cell.next_pressure + pressure_diff;
            cell.next_pressure = 0;
            cell.can_flow_out = false;
        }
    }
    *map = Map::new_from_iter(iter);
}

fn advance_fluid_upwards(map: &mut Map) {
    let yp = CellIndex::new(0, 1, 0);
    let yn = CellIndex::new(0, -1, 0);
    let pressure_threshold = VERTICAL_PRESSURE_DIFFERENCE + 1;
    let flow = Flow::new(map, pressure_threshold);
    let updated_map = map.clone();
    let mut iter = updated_map.iter_mut();
    while let Option::Some(CellIterItem { cell_index, cell }) = iter.next() {
        if is_liquid(cell.tile_type) {
            let current_pressure = cell.pressure;
            let mut pressure_diff = 0;
            flow.flow_outwards(cell_index + yp, current_pressure, &mut pressure_diff);
            flow.flow_inwards(cell_index + yn, current_pressure, &mut pressure_diff);
            cell.pressure += pressure_diff;
        }
    }
    *map = Map::new_from_iter(iter);
}

struct Flow<'a> {
    map: &'a Map,
    pressure_threshold: Pressure,
}

impl<'a> Flow<'a> {
    pub fn new(map: &'a Map, pressure_threshold: Pressure) -> Self {
        Flow { map, pressure_threshold }
    }

    fn flow_outwards(
        &self,
        adjacent_index: CellIndex,
        current_pressure: Pressure,
        pressure_diff: &mut Pressure,
    ) {
        if is_valid(adjacent_index, self.map) {
            let adjacent_cell = self.map.get_cell(adjacent_index);
            if (current_pressure - adjacent_cell.pressure) > self.pressure_threshold {
                *pressure_diff -= 1;
            }
        }
    }

    fn flow_inwards(&self,
        adjacent_index: CellIndex,
        current_pressure: Pressure,
        pressure_diff: &mut Pressure,
    ) {
        if is_valid(adjacent_index, self.map) {
            let adjacent_cell = self.map.get_cell(adjacent_index);
            if (adjacent_cell.pressure - current_pressure) > self.pressure_threshold {
                *pressure_diff += 1;
            }
        }
    }

    fn maybe_flow_inwards(&self,
        adjacent_index: CellIndex,
        current_pressure: Pressure,
        pressure_diff: &mut Pressure,
    ) {
        if is_valid(adjacent_index, self.map) {
            let adjacent_cell = self.map.get_cell(adjacent_index);
            if adjacent_cell.can_flow_out
                    && ((adjacent_cell.pressure - current_pressure) > self.pressure_threshold) {
                *pressure_diff += 1;
            }
        }
    }
}


// TODO: optimize one cell fetch. here and where this is called we do a redundant get_cell
fn is_valid(cell_index: CellIndex, map: &Map) -> bool {
    map.in_range(cell_index) && is_liquid_or_air(map.get_cell(cell_index).tile_type)
}

fn swap_next_pressure_to_current(map: &mut Map, min_cell: CellIndex, max_cell: CellIndex) {
    let iter = CellCubeIterator::new(min_cell, max_cell);
    for cell_index in iter {
        let nothing_above = {
            let above_cell = map.get_cell_mut(cell_index + CellIndex::new(0, 1, 0));
            ((above_cell.pressure + above_cell.next_pressure) <= 0)
                && is_liquid_or_air(above_cell.tile_type)
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
            } else if nothing_above {
                // if pressure_above > 0 {
                //     println!("above cell should be air!");
                // }
                TileType::DirtyWaterSurface
            } else {
                TileType::DirtyWaterWall
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
            50, 11, 11,
            44, -1, 0,
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
            32, 29, 30,
            20, -1, 19,
            11, -1, 9,
        ];
        assert_n_steps(cells.clone(), expected, 90, min_cell, max_cell);

        #[rustfmt::skip]
        let expected = vec![
            31, 31, 29,
            21, -1, 19,
            10, -1, 9,
        ];
        assert_n_steps(cells.clone(), expected, 91, min_cell, max_cell);

        #[rustfmt::skip]
        let expected = vec![
            31, 30, 30,
            21, -1, 19,
            10, -1, 9,
        ];
        assert_n_steps(cells.clone(), expected, 92, min_cell, max_cell);
    }
}
