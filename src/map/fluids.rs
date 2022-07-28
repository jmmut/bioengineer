use crate::map::chunk::cell_iter::CellIterItem;
use crate::map::fluids::FluidStage::Downwards;
use crate::map::ref_mut_iterator::RefMutIterator;
use crate::map::{
    cell::is_liquid, cell::is_liquid_or_air, cell::Pressure, Cell, CellCubeIterator, CellIndex,
    Map, TileType,
};
use crate::ScopedProfiler;

const VERTICAL_PRESSURE_DIFFERENCE: i32 = 10;

pub struct Fluids {
    mode: FluidMode,
    next_stage: FluidStage,
    profile: bool,
}

#[allow(unused)]
pub enum FluidMode {
    AllTogether,
    InStages,
}

#[derive(Copy, Clone, Debug)]
enum FluidStage {
    Downwards,
    SidewaysPrepare,
    SidewaysApply,
    Upwards,
    TileUpdate,
}
impl Fluids {
    pub fn new(mode: FluidMode) -> Self {
        Self {
            mode,
            next_stage: Downwards,
            profile: false,
        }
    }
    pub fn advance(&mut self, map: &mut Map) {
        match self.mode {
            FluidMode::AllTogether => advance_fluid(map),
            FluidMode::InStages => {
                self.advance_fluid_stage(map);
                self.next_stage = next_fluid_stage(self.next_stage);
            }
        }
    }
    pub fn set_profile(&mut self, profile: bool) {
        self.profile = profile;
    }
    fn advance_fluid_stage(&mut self, map: &mut Map) {
        use FluidStage::*;
        let _profiler = self.maybe_profile();
        match self.next_stage {
            Downwards => advance_fluid_downwards(map),
            SidewaysPrepare => prepare_fluid_sideways(map),
            SidewaysApply => advance_fluid_sideways(map),
            Upwards => advance_fluid_upwards(map),
            TileUpdate => update_tile_type(map),
        }
    }

    fn maybe_profile(&mut self) -> ScopedProfiler {
        let profile_name = format!("fluid stage: {:?}", self.next_stage);
        let profiler = ScopedProfiler::new_named(self.profile, profile_name.as_str());
        profiler
    }
}

fn next_fluid_stage(stage: FluidStage) -> FluidStage {
    use FluidStage::*;
    match stage {
        Downwards => SidewaysPrepare,
        SidewaysPrepare => SidewaysApply,
        SidewaysApply => Upwards,
        Upwards => TileUpdate,
        TileUpdate => Downwards,
    }
}

fn advance_fluid(map: &mut Map) {
    advance_fluid_downwards(map);
    prepare_fluid_sideways(map);
    advance_fluid_sideways(map);
    advance_fluid_upwards(map);
    update_tile_type(map);
}

fn advance_fluid_downwards(map: &mut Map) {
    #[cfg(test)]
    println!("advancing");
    #[cfg(test)]
    print_map_pressures(map);

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

    #[cfg(test)]
    print_map_pressures(map);
}

fn prepare_fluid_sideways(map: &mut Map) {
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

    #[cfg(test)]
    print_map_pressures(map);
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

    #[cfg(test)]
    print_map_pressures(map);
}

struct Flow<'a> {
    map: &'a Map,
    pressure_threshold: Pressure,
}

impl<'a> Flow<'a> {
    pub fn new(map: &'a Map, pressure_threshold: Pressure) -> Self {
        Flow {
            map,
            pressure_threshold,
        }
    }

    fn flow_outwards(
        &self,
        adjacent_index: CellIndex,
        current_pressure: Pressure,
        pressure_diff: &mut Pressure,
    ) {
        if current_pressure > 0 {
            let option_cell = is_valid(adjacent_index, self.map);
            if let Option::Some(adjacent_cell) = option_cell {
                if (current_pressure - adjacent_cell.pressure) > self.pressure_threshold {
                    *pressure_diff -= 1;
                }
            }
        }
    }

    fn flow_inwards(
        &self,
        adjacent_index: CellIndex,
        current_pressure: Pressure,
        pressure_diff: &mut Pressure,
    ) {
        let option_cell = is_valid(adjacent_index, self.map);
        if let Option::Some(adjacent_cell) = option_cell {
            if adjacent_cell.pressure > 0
                && (adjacent_cell.pressure - current_pressure) > self.pressure_threshold
            {
                *pressure_diff += 1;
            }
        }
    }

    fn maybe_flow_inwards(
        &self,
        adjacent_index: CellIndex,
        current_pressure: Pressure,
        pressure_diff: &mut Pressure,
    ) {
        let option_cell = is_valid(adjacent_index, self.map);
        if let Option::Some(adjacent_cell) = option_cell {
            if adjacent_cell.can_flow_out
                && ((adjacent_cell.pressure - current_pressure) > self.pressure_threshold)
            {
                *pressure_diff += 1;
            }
        }
    }
}

fn is_valid(cell_index: CellIndex, map: &Map) -> Option<Cell> {
    let option_cell = map.get_cell_optional(cell_index);
    return if let Option::Some(cell) = option_cell {
        if is_liquid_or_air(cell.tile_type) {
            Option::Some(*cell)
        } else {
            Option::None
        }
    } else {
        Option::None
    };
}

fn update_tile_type(map: &mut Map) {
    let updated_map = map.clone();
    let mut iter = updated_map.iter_mut();
    while let Option::Some(CellIterItem { cell_index, cell }) = iter.next() {
        if is_liquid_or_air(cell.tile_type) {
            let nothing_above = {
                let index_above = cell_index + CellIndex::new(0, 1, 0);
                let option_above_cell = map.get_cell_optional(index_above);
                if let Option::Some(above_cell) = option_above_cell {
                    (above_cell.pressure <= 0) && is_liquid_or_air(above_cell.tile_type)
                } else {
                    false
                }
            };
            if is_liquid_or_air(cell.tile_type) {
                if cell.pressure < 0 {
                    panic!(
                        "negative pressure! for cell {}, with pressure {}, next pressure {}.",
                        cell_index, cell.pressure, cell.next_pressure
                    );
                }
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
            }
        }
    }
    *map = Map::new_from_iter(iter);
}

#[allow(unused)]
fn print_map_pressures(map: &Map) {
    let iter = CellCubeIterator::new(map.min_cell(), map.max_cell());
    let mut pressures = Vec::new();
    for cell_index in iter {
        pressures.push(map.get_cell(cell_index).pressure)
    }
    println!("pressures: {:?}", pressures);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::map::Map;
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
        let mut assert_until = |
            initial_map: Vec<Pressure>,
            final_map: Vec<Pressure>,
            iterations: i32,
        | -> Vec<Pressure> {
            let computed = assert_n_steps(initial_map, final_map, iterations - i, min_cell, max_cell);
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

        cells = assert_until(cells, final_expected_loop, 95);
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
            &cells.clone(),
            iterations,
            FluidMode::AllTogether,
        );
        let computed_in_stages = compute_n_steps(
            min_cell,
            max_cell,
            &cells.clone(),
            iterations * 5,
            FluidMode::InStages,
        );

        assert_eq!(computed_in_stages, computed_together);
    }

    fn compute_n_steps(
        min_cell: IVec3,
        max_cell: IVec3,
        cells: &Vec<i32>,
        iterations: i32,
        mode: FluidMode,
    ) -> Vec<i32> {
        let mut map = Map::_new_from_pressures(cells.clone(), min_cell, max_cell);
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
}
