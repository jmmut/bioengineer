pub mod cell;
mod cell_cube_iterator;
pub mod cell_envelope;
pub mod chunk;
mod map_iterator;
pub mod ref_mut_iterator;
pub mod transform_cells;

use crate::common::trunc::trunc_towards_neg_inf;
use crate::world::fluids::VERTICAL_PRESSURE_DIFFERENCE;
use cell::Pressure;
pub use cell::{
    is_covering, is_liquid_or_air, is_walkable_horizontal, is_walkable_vertical, Cell, TileType,
};
pub use cell_cube_iterator::CellCubeIterator;
use cell_envelope::Envelope;
use chunk::chunks::Chunks;
use chunk::Chunk;
use chunk::{get_chunk_index, get_required_chunks};
use map_iterator::MutMapIterator;
use mq_basics::{now, IVec3};
use opensimplex_noise_rs::OpenSimplexNoise;
use std::cmp::Ordering;

/// The axis are isometric:
/// - x: right towards camera
/// - y: up
/// - z: left towards camera
pub type CellIndex = IVec3;
pub type PressureAndType = (Pressure, TileType);

const MAP_SIZE: i32 = 64;

#[derive(Clone)]
pub struct Map {
    chunks: Chunks,
    min_cell: CellIndex,
    max_cell: CellIndex,
    ship_position: Option<CellIndex>,
}

impl Map {
    pub fn new() -> Self {
        Self::new_for_cube(Self::default_min_cell(), Self::default_max_cell())
    }

    pub fn new_for_cube(min_cell: CellIndex, max_cell: CellIndex) -> Self {
        let mut chunks = Chunks::new();
        let chunk_indexes = get_required_chunks(min_cell, max_cell);
        for chunk_index in chunk_indexes {
            chunks.insert(chunk_index, Chunk::new_from_chunk_index(chunk_index));
        }
        let ship_position = Option::None;
        Map {
            chunks,
            min_cell,
            max_cell,
            ship_position,
        }
    }

    pub fn _new_from_pressures(
        cells: Vec<Pressure>,
        min_cell: CellIndex,
        max_cell: CellIndex,
    ) -> Self {
        let mut map = Self::new_for_cube(min_cell, max_cell);
        for (i, cell_index) in CellCubeIterator::new(min_cell, max_cell).enumerate() {
            map.get_cell_mut(cell_index).pressure = cells[i];
            map.get_cell_mut(cell_index).tile_type = if cells[i] >= 0 {
                TileType::DirtyWaterWall
            } else {
                TileType::WallRock
            };
        }
        map
    }

    pub fn _new_from_pressures_and_tiles(
        cells: Vec<PressureAndType>,
        min_cell: CellIndex,
        max_cell: CellIndex,
    ) -> Self {
        let mut map = Self::new_for_cube(min_cell, max_cell);
        for (i, cell_index) in CellCubeIterator::new(min_cell, max_cell).enumerate() {
            (
                map.get_cell_mut(cell_index).pressure,
                map.get_cell_mut(cell_index).tile_type,
            ) = cells[i];
        }
        map
    }

    pub fn _new_from_tiles(default_cell: Cell, tiles: Vec<(CellIndex, TileType)>) -> Self {
        let mut chunks = Chunks::new();
        let mut envelope = Envelope::new();
        for (cell_index, _tile) in &tiles {
            let chunk_indexes = get_required_chunks(*cell_index, *cell_index);
            for chunk_index in chunk_indexes {
                let chunk =
                    Chunk::new_from_chunk_index_with_default_cell(chunk_index, default_cell);
                chunks.insert(chunk_index, chunk);
            }
            envelope.add(cell_index);
        }
        let (min_cell, max_cell) = envelope.result();
        let ship_position = Option::None;
        let mut map = Map {
            chunks,
            min_cell,
            max_cell,
            ship_position,
        };
        for (cell_index, tile) in tiles {
            map.get_cell_mut(cell_index).tile_type = tile;
        }
        map
    }

    pub fn new_from_iter(mut_map_iter: MutMapIterator) -> Self {
        Self {
            chunks: mut_map_iter.collected_chunks,
            min_cell: mut_map_iter.min_cell,
            max_cell: mut_map_iter.max_cell,
            ship_position: mut_map_iter.ship_position,
        }
    }

    pub fn default_min_cell() -> CellIndex {
        CellIndex::new(-MAP_SIZE / 2, -MAP_SIZE / 2, -MAP_SIZE / 2)
    }
    pub fn default_max_cell() -> CellIndex {
        CellIndex::new(MAP_SIZE / 2 - 1, MAP_SIZE / 2 - 1, MAP_SIZE / 2 - 1)
    }
    #[allow(unused)]
    pub fn min_cell(&self) -> CellIndex {
        self.min_cell
    }
    #[allow(unused)]
    pub fn max_cell(&self) -> CellIndex {
        self.max_cell
    }

    /// Don't use this if you plan to use the cell. Use get_cell_optional() instead
    #[allow(unused)]
    pub fn in_range(&self, cell_index: CellIndex) -> bool {
        self.get_chunk_optional(&cell_index).is_some()
    }
    pub fn get_cell(&self, index: CellIndex) -> &Cell {
        self.get_chunk(index).get_cell(index)
    }
    pub fn get_cell_optional(&self, index: CellIndex) -> Option<&Cell> {
        self.get_chunk_optional(&index)
            .map(|chunk| chunk.get_cell(index))
    }
    pub fn get_cell_mut(&mut self, index: CellIndex) -> &mut Cell {
        self.get_chunk_mut(index).get_cell_mut(index)
    }

    fn get_chunk(&self, index: CellIndex) -> &Chunk {
        self.get_chunk_optional(&index)
            .expect("Error: Making the map bigger dynamically is disabled.")
    }

    fn get_chunk_optional(&self, index: &CellIndex) -> Option<&Chunk> {
        self.chunks.get(&get_chunk_index(index))
    }

    #[allow(dead_code)]
    fn get_chunk_mut(&mut self, index: CellIndex) -> &mut Chunk {
        self.chunks
            .get_mut(&get_chunk_index(&index))
            .expect("Error: Making the map bigger dynamically is disabled.")
    }

    pub fn regenerate(&mut self) {
        #[allow(unused)]
        enum MapType {
            Island,
            Simplex,
        }
        let map_type = MapType::Island;
        match map_type {
            MapType::Island => self.regenerate_island(),
            MapType::Simplex => self.regenerate_with_simplex_noise(),
        };
    }

    fn regenerate_island(&mut self) {
        for (chunk_index, chunk) in &mut self.chunks {
            for cell_index in chunk.iter(*chunk_index) {
                let cell = chunk.get_cell_mut(cell_index);
                choose_tile_in_island_map(cell_index, cell)
            }
        }
        let ship_pos = CellIndex::new(0, 1, 0);
        self.get_cell_mut(ship_pos).tile_type = TileType::MachineShip;
        self.ship_position = Option::Some(ship_pos);
    }

    fn regenerate_with_simplex_noise(&mut self) {
        // if not provided, default seed is equal to 0
        let noise_generator = OpenSimplexNoise::new(Some(now() as i64));
        let scale = 0.08;
        let mut min = 0.0;
        let mut max = 0.0;
        for (chunk_index, chunk) in &mut self.chunks {
            for cell_index in chunk.iter(*chunk_index) {
                // -1 to 1
                let value = noise_generator
                    .eval_2d(cell_index.x as f64 * scale, cell_index.z as f64 * scale);
                if value > max {
                    max = value;
                }
                if value < min {
                    min = value
                }
                let tile = choose_tile(value, cell_index);
                let cell = chunk.get_cell_mut(cell_index);
                if is_liquid_or_air(tile) {
                    use VERTICAL_PRESSURE_DIFFERENCE as PRESSURE;
                    cell.pressure = i32::max(0, PRESSURE - PRESSURE * cell_index.y);
                    // cell.pressure = if tile == TileType::Air { 0 } else {40};
                }
                cell.tile_type = tile
            }
        }
        println!("simplex range used: [{}, {}]", min, max);
    }

    pub fn get_ship_position(&self) -> Option<CellIndex> {
        self.ship_position
    }

    pub fn _get_pressures(&self, min_cell: CellIndex, max_cell: CellIndex) -> Vec<Pressure> {
        let mut cells = Vec::new();
        for cell_index in CellCubeIterator::new(min_cell, max_cell) {
            cells.push(self.get_cell(cell_index).pressure);
        }
        cells
    }

    pub fn _get_pressures_and_types(
        &self,
        min_cell: CellIndex,
        max_cell: CellIndex,
    ) -> Vec<PressureAndType> {
        let mut cells = Vec::new();
        for cell_index in CellCubeIterator::new(min_cell, max_cell) {
            let cell = self.get_cell(cell_index);
            cells.push((cell.pressure, cell.tile_type));
        }
        cells
    }

    pub fn iter_mut(self) -> MutMapIterator {
        MutMapIterator::new(
            self.chunks,
            self.min_cell,
            self.max_cell,
            self.ship_position,
        )
    }
}

fn choose_tile_in_island_map(cell_index: CellIndex, cell: &mut Cell) {
    cell.pressure = 0;
    cell.can_flow_out = false;
    cell.next_pressure = 0;
    if cell_index.y > 1 {
        cell.tile_type = TileType::Air;
    } else {
        let horizontal_distance_from_center =
            f32::sqrt((cell_index.x * cell_index.x + cell_index.z * cell_index.z) as f32);
        let island_radius = 5.0;
        let steepness = 4.0;
        let enlargement_by_deepness = -cell_index.y as f32 / steepness;
        let is_land = horizontal_distance_from_center < island_radius + enlargement_by_deepness;
        if is_land {
            cell.tile_type = if cell_index.y == 1 {
                TileType::FloorDirt
            } else {
                TileType::WallRock
            };
        } else {
            cell.tile_type = match cell_index.y.cmp(&0) {
                Ordering::Greater => TileType::Air,
                Ordering::Less => TileType::DirtyWaterWall,
                Ordering::Equal => TileType::DirtyWaterSurface,
            };
            use VERTICAL_PRESSURE_DIFFERENCE as PRESSURE;
            cell.pressure = i32::max(0, PRESSURE - PRESSURE * cell_index.y);
        }
    }
}

fn choose_tile(value: f64, cell_index: CellIndex) -> TileType {
    use TileType::*;
    let terrain_height = trunc_towards_neg_inf((value * 0.5 * MAP_SIZE as f64) as i32, 2);
    match cell_index.y.cmp(&terrain_height) {
        Ordering::Less => WallRock,
        Ordering::Equal => FloorDirt,
        Ordering::Greater => {
            let water_height = 0;
            match cell_index.y.cmp(&water_height) {
                Ordering::Less => DirtyWaterWall,
                Ordering::Equal => DirtyWaterSurface,
                Ordering::Greater => Air,
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_in_range() {
        let map = Map::new();
        assert_eq!(map.in_range(CellIndex::new(0, 0, 0)), true);
        assert_eq!(map.in_range(CellIndex::new(0, 0, -MAP_SIZE)), false);
    }

    #[test]
    fn test_new_from_tiles_basic() {
        let map = Map::_new_from_tiles(
            Cell::new(TileType::FloorDirt),
            vec![(CellIndex::new(0, 0, 0), TileType::FloorRock)],
        );
        assert_eq!(map.chunks.len(), 1);
    }
    #[test]
    fn test_new_from_tiles() {
        let some_pos = CellIndex::new(0, 0, 0);
        let other_pos = CellIndex::new(0, 100, 0);
        let some_tile = TileType::FloorRock;
        let other_tile = TileType::WallRock;
        let default_cell = Cell::new(TileType::FloorDirt);
        let map = Map::_new_from_tiles(
            default_cell,
            vec![(some_pos, some_tile), (other_pos, other_tile)],
        );
        assert_eq!(map.chunks.len(), 2);
        assert_eq!(*map.get_cell(some_pos), Cell::new(some_tile));
        assert_eq!(*map.get_cell(other_pos), Cell::new(other_tile));
        assert_eq!(
            *map.get_cell(other_pos + CellIndex::new(1, 0, 0)),
            default_cell
        );
    }
}
