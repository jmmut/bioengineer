#[cfg(test)]
mod tests;

use crate::common::profiling::ScopedProfiler;
use crate::world::map::cell::{
    is_floodable_from_above, is_floodable_from_below, is_floodable_horizontal,
};
use crate::world::map::chunk::cell_iter::CellIterItem;
use crate::world::map::ref_mut_iterator::RefMutIterator;
use crate::world::map::{cell::Pressure, Cell, CellCubeIterator, CellIndex, Map};
use mq_basics::IVec3;

pub const VERTICAL_PRESSURE_DIFFERENCE: i32 = 10;

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
            next_stage: FluidStage::Downwards,
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
    println!("advancing (before down)");
    #[cfg(test)]
    print_map_pressures(map, "before down");

    let yp = CellIndex::new(0, 1, 0);
    let yn = CellIndex::new(0, -1, 0);
    let pressure_threshold = -VERTICAL_PRESSURE_DIFFERENCE;
    let flow = Flow::new(map, pressure_threshold);
    let updated_map = map.clone();
    let mut iter = updated_map.iter_mut();
    while let Option::Some(CellIterItem { cell_index, cell }) = iter.next() {
        if is_floodable_from_above(cell.tile_type) {
            let current_pressure = cell.pressure;
            let mut pressure_diff = 0;
            flow.flow_outwards(cell_index, yn, current_pressure, &mut pressure_diff);
            flow.flow_inwards(cell_index, yp, current_pressure, &mut pressure_diff);
            cell.pressure += pressure_diff;
        }
    }
    *map = Map::new_from_iter(iter);

    #[cfg(test)]
    print_map_pressures(map, "after down");
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
        if is_floodable_horizontal(cell.tile_type) {
            let current_pressure = cell.pressure;
            let mut pressure_diff = 0;
            flow.flow_outwards(cell_index, xp, current_pressure, &mut pressure_diff);
            flow.flow_outwards(cell_index, xn, current_pressure, &mut pressure_diff);
            flow.flow_outwards(cell_index, zp, current_pressure, &mut pressure_diff);
            flow.flow_outwards(cell_index, zn, current_pressure, &mut pressure_diff);
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
        if is_floodable_horizontal(cell.tile_type) {
            let current_pressure = cell.pressure;
            let mut pressure_diff = 0;
            flow.maybe_flow_inwards(cell_index, xp, current_pressure, &mut pressure_diff);
            flow.maybe_flow_inwards(cell_index, xn, current_pressure, &mut pressure_diff);
            flow.maybe_flow_inwards(cell_index, zp, current_pressure, &mut pressure_diff);
            flow.maybe_flow_inwards(cell_index, zn, current_pressure, &mut pressure_diff);
            cell.pressure = cell.next_pressure + pressure_diff;
            cell.next_pressure = 0;
            cell.can_flow_out = false;
        }
    }
    *map = Map::new_from_iter(iter);

    #[cfg(test)]
    print_map_pressures(map, "after sideways");
}

fn advance_fluid_upwards(map: &mut Map) {
    let yp = CellIndex::new(0, 1, 0);
    let yn = CellIndex::new(0, -1, 0);
    let pressure_threshold = VERTICAL_PRESSURE_DIFFERENCE + 1;
    let flow = Flow::new(map, pressure_threshold);
    let updated_map = map.clone();
    let mut iter = updated_map.iter_mut();
    while let Option::Some(CellIterItem { cell_index, cell }) = iter.next() {
        if is_floodable_from_below(cell.tile_type) {
            let current_pressure = cell.pressure;
            let mut pressure_diff = 0;
            flow.flow_outwards(cell_index, yp, current_pressure, &mut pressure_diff);
            flow.flow_inwards(cell_index, yn, current_pressure, &mut pressure_diff);
            cell.pressure += pressure_diff;
        }
    }
    *map = Map::new_from_iter(iter);

    #[cfg(test)]
    print_map_pressures(map, "after upwards");
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
        target_index: CellIndex,
        diff: CellIndex,
        current_pressure: Pressure,
        pressure_diff: &mut Pressure,
    ) {
        if current_pressure > 0 {
            let option_cell = is_floodable(target_index, diff, self.map, true);
            if let Option::Some(adjacent_cell) = option_cell {
                if (current_pressure - adjacent_cell.pressure) > self.pressure_threshold {
                    *pressure_diff -= 1;
                }
            }
        }
    }

    fn flow_inwards(
        &self,
        target_index: CellIndex,
        diff: CellIndex,
        current_pressure: Pressure,
        pressure_diff: &mut Pressure,
    ) {
        let option_cell = is_floodable(target_index, diff, self.map, false);
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
        target_index: CellIndex,
        diff: CellIndex,
        current_pressure: Pressure,
        pressure_diff: &mut Pressure,
    ) {
        let option_cell = is_floodable(target_index, diff, self.map, false);
        if let Option::Some(adjacent_cell) = option_cell {
            if adjacent_cell.can_flow_out
                && ((adjacent_cell.pressure - current_pressure) > self.pressure_threshold)
            {
                *pressure_diff += 1;
            }
        }
    }
}

fn is_floodable(cell_index: CellIndex, diff: CellIndex, map: &Map, outwards: bool) -> Option<Cell> {
    let option_cell = map.get_cell_optional(cell_index + diff);
    return if let Option::Some(cell) = option_cell {
        let floodable = if diff == IVec3::Y {
            if outwards {
                is_floodable_from_below(cell.tile_type)
            } else {
                is_floodable_from_above(cell.tile_type)
            }
        } else if diff == IVec3::NEG_Y {
            if outwards {
                is_floodable_from_above(cell.tile_type)
            } else {
                is_floodable_from_below(cell.tile_type)
            }
        } else {
            is_floodable_horizontal(cell.tile_type)
        };
        if floodable {
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
        // if is_floodable_horizontal(cell.tile_type) {
        // let nothing_above = {
        //     let index_above = cell_index + CellIndex::new(0, 1, 0);
        //     let option_above_cell = map.get_cell_optional(index_above);
        //     if let Option::Some(above_cell) = option_above_cell {
        //         (above_cell.pressure <= 0) && is_liquid_or_air(above_cell.tile_type)
        //     } else {
        //         false
        //     }
        // };
        if cell.pressure < 0 {
            panic!(
                "negative pressure! for cell {}, with pressure {}, next pressure {}.",
                cell_index, cell.pressure, cell.next_pressure
            );
        }
        // let new_type = if cell.pressure <= 0 {
        //     TileType::Air
        // } else if cell.pressure <= VERTICAL_PRESSURE_DIFFERENCE {
        //     // if pressure_above > 0 {
        //     //     println!("above cell should be air!");
        //     // }
        //     TileType::DirtyWaterSurface
        // } else {
        //     TileType::DirtyWaterWall
        // };
        // // if cell_index == CellIndex::new(0, 1, 5) {
        // //     println!(
        // //         "cell with pressure {}, is {:?}, converted to {:?}",
        // //         cell.pressure, cell.tile_type, new_type
        // //     );
        // // }
        // cell.tile_type = new_type;
        cell.renderable_pressure = cell.pressure;
        // }
    }
    *map = Map::new_from_iter(iter);
}

#[allow(unused)]
fn print_map_pressures(map: &Map, name: &str) {
    let iter = CellCubeIterator::new(map.min_cell(), map.max_cell());
    let mut pressures = Vec::new();
    for cell_index in iter {
        pressures.push(map.get_cell(cell_index).pressure)
    }
    println!("pressures {}: {:?}", name, pressures);
}
