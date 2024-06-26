pub mod cell;
mod cell_cube_iterator;
pub mod cell_envelope;
pub mod chunk;
mod map_iterator;
pub mod ref_mut_iterator;
pub mod transform_cells;
pub mod transformation_rules;

use crate::common::trunc::trunc_towards_neg_inf;
use crate::world::fluids::VERTICAL_PRESSURE_DIFFERENCE;
use crate::world::robots::DOWN;
pub use cell::{
    is_covering, is_liquid_or_air, is_walkable_horizontal, is_walkable_vertical, Cell, Pressure,
    TileType,
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

/// The axes are isometric:
/// - x: right towards camera
/// - y: up
/// - z: left towards camera
pub type CellIndex = IVec3;
pub type PressureAndType = (Pressure, TileType);

const MAP_SIZE: i32 = 64;
pub const DEFAULT_MAP_TYPE: MapType = MapType::Island;

#[derive(Clone)]
pub struct Map {
    chunks: Chunks,
    min_cell: CellIndex,
    max_cell: CellIndex,
    ship_position: Option<CellIndex>,
    map_type: MapType,
}

#[derive(Copy, Clone)]
pub enum MapType {
    Island,
    Simplex,
}

impl Map {
    pub fn new() -> Self {
        Self::new_for_cube(Self::default_min_cell(), Self::default_max_cell())
    }

    pub fn new_generated(map_type: MapType) -> Self {
        let mut map = Self::new();
        map.map_type = map_type;
        map.regenerate();
        map
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
            map_type: DEFAULT_MAP_TYPE,
        }
    }

    pub fn _new_from_pressures(
        cells: Vec<Pressure>,
        min_cell: CellIndex,
        max_cell: CellIndex,
    ) -> Self {
        let mut map = Self::new_for_cube(min_cell, max_cell);
        for (i, cell_index) in CellCubeIterator::new(min_cell, max_cell).enumerate() {
            map.get_cell_mut(cell_index).pressure = if cells[i] >= 0 { cells[i] } else { 0 };
            map.get_cell_mut(cell_index).tile_type = if cells[i] >= 0 {
                TileType::Air
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
            map_type: DEFAULT_MAP_TYPE,
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
            map_type: mut_map_iter.map_type,
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
        match self.map_type {
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
        let scale = 0.12;
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
                let cell = chunk.get_cell_mut(cell_index);
                choose_tile_simplex(value, cell_index, cell);
            }
        }
        let mut ship_pos = None;
        for x in -5..15 {
            ship_pos = self.land_ship(x, 0);
            if ship_pos.is_some() {
                break;
            }
        }
        if let Some(pos) = ship_pos {
            self.get_cell_mut(pos).tile_type = TileType::MachineShip;
            self.ship_position = Some(pos);
        } else {
            panic!("couldn't land ship");
        }
        println!("simplex range used: [{}, {}]", min, max);
    }
    pub fn land_ship(&self, x: i32, z: i32) -> Option<CellIndex> {
        let mut ship_pos = CellIndex::new(x, self.max_cell.y, z);
        let mut below = ship_pos + DOWN;
        let mut below_cell = self.get_cell(below);
        while below_cell.tile_type == TileType::Air {
            if below_cell.pressure > 0 {
                return None;
            }
            ship_pos = below;
            below += DOWN;
            below_cell = self.get_cell(below);
        }
        Some(ship_pos)
    }

    pub fn get_ship_position(&self) -> Option<CellIndex> {
        self.ship_position
    }

    pub fn _get_pressures(&self, min_cell: CellIndex, max_cell: CellIndex) -> Vec<Pressure> {
        let mut cells = Vec::new();
        for cell_index in CellCubeIterator::new(min_cell, max_cell) {
            let cell = self.get_cell(cell_index);
            cells.push(if cell.tile_type == TileType::WallRock {
                -1
            } else {
                cell.pressure
            });
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
            self.map_type,
        )
    }
}

fn choose_tile_in_island_map(cell_index: CellIndex, cell: &mut Cell) {
    cell.tile_type = if cell_index.y > 1 {
        TileType::Air
    } else {
        let horizontal_distance_from_center =
            f32::sqrt((cell_index.x * cell_index.x + cell_index.z * cell_index.z) as f32);
        let island_radius = 5.0;
        let steepness = 4.0;
        let enlargement_by_deepness = -cell_index.y as f32 / steepness;
        let is_land = horizontal_distance_from_center < island_radius + enlargement_by_deepness;
        if is_land {
            if cell_index.y == 1 {
                TileType::Air
            } else {
                TileType::WallRock
            }
        } else {
            TileType::Air
        }
    };
    define_pressure(cell.tile_type, cell_index, cell);
}

fn choose_tile_simplex(value: f64, cell_index: CellIndex, cell: &mut Cell) {
    use TileType::*;
    let terrain_height = trunc_towards_neg_inf((value * 0.5 * MAP_SIZE as f64) as i32, 2);
    let tile_type = match cell_index.y.cmp(&terrain_height) {
        Ordering::Less => WallRock,
        Ordering::Equal => WallDirt,
        Ordering::Greater => {
            use VERTICAL_PRESSURE_DIFFERENCE as PRESSURE;
            cell.pressure = i32::max(0, PRESSURE - PRESSURE * cell_index.y);
            cell.renderable_pressure = cell.pressure;
            Air
        }
    };
    cell.tile_type = tile_type;
    define_pressure(cell.tile_type, cell_index, cell);
}

fn define_pressure(tile_type: TileType, cell_index: CellIndex, cell: &mut Cell) {
    cell.pressure = 0;
    cell.can_flow_out = false;
    cell.next_pressure = 0;
    if tile_type == TileType::Air {
        use VERTICAL_PRESSURE_DIFFERENCE as PRESSURE;
        cell.pressure = i32::max(0, PRESSURE - PRESSURE * cell_index.y);
        cell.renderable_pressure = cell.pressure;
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
