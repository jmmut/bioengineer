use crate::world::map::cell::{is_sturdy, is_tree};
use crate::world::map::transform_cells::{above, above_is, below, below_is, TransformationResult};
use crate::world::map::{CellIndex, Map, TileType};

pub struct TransformationRules<'a> {
    position_to_transform: CellIndex,
    new_tile_type: TileType,
    map: &'a Map,
}

impl<'a> TransformationRules<'a> {
    pub fn new(position_to_transform: CellIndex, new_tile_type: TileType, map: &'a Map) -> Self {
        Self {
            position_to_transform,
            new_tile_type,
            map,
        }
    }
    pub fn is_forbidden(&self) -> Option<TransformationResult> {
        if self.cells_above_would_collapse() {
            Some(TransformationResult::AboveWouldCollapse)
        } else if self.building_on_top_of_non_sturdy_cells() {
            Some(TransformationResult::NoSturdyBase)
        } else if self.planting_tree_on_non_soil() {
            Some(TransformationResult::NoSturdyBase)
        } else if self.occluding_solar_panel() {
            Some(TransformationResult::WouldOccludeSolarPanel)
        } else {
            None
        }
    }

    pub fn cells_above_would_collapse(&self) -> bool {
        !is_sturdy(self.new_tile_type)
            && !above_is(TileType::Air, self.position_to_transform, self.map)
            || self.new_tile_type != TileType::WallRock
                && above(is_tree, self.position_to_transform, self.map)
    }
    pub fn building_on_top_of_non_sturdy_cells(&self) -> bool {
        self.new_tile_type != TileType::Air
            && !below(is_sturdy, self.position_to_transform, self.map)
    }
    pub fn planting_tree_on_non_soil(&self) -> bool {
        self.new_tile_type == TileType::TreeHealthy
            && !below_is(TileType::WallRock, self.position_to_transform, self.map)
    }
    pub fn occluding_solar_panel(&self) -> bool {
        self.new_tile_type != TileType::Air
            && below_is(
                TileType::MachineSolarPanel,
                self.position_to_transform,
                self.map,
            )
    }
}
