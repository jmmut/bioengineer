mod chunk;
mod fluids;
mod map_iterator;
pub mod transform_cells;
pub mod trunc;
mod ref_mut_iterator;
mod cell_cube_iterator;
mod cell;

use crate::map::chunk::{get_chunk_index, get_required_chunks};
use crate::{now, IVec3};
use chunk::{Chunk, ChunkIndex};
use opensimplex_noise_rs::OpenSimplexNoise;
use std::cmp::{max, min};
use std::collections::HashMap;
use trunc::trunc_towards_neg_inf;

// use crate::map::map_iterator::MapIterator;
pub use crate::map::cell::{Cell, is_liquid_or_air, TileType};
pub use crate::map::cell_cube_iterator::CellCubeIterator;
use crate::map::map_iterator::MutMapIterator;

/// The axis are isometric:
/// - x: right towards camera
/// - y: up
/// - z: left towards camera
pub type CellIndex = IVec3;

const MAP_SIZE: i32 = 21;

pub struct Map {
    chunks: HashMap<ChunkIndex, Chunk>,
    min_cell: CellIndex,
    max_cell: CellIndex,
}

impl Map {
    pub fn in_range(&self, cell_index: CellIndex) -> bool {
        self.get_chunk_optional(&cell_index).is_some()
    }
}

impl Map {
    pub fn new() -> Self {
        Self::new_for_cube(Self::default_min_cell(), Self::default_max_cell())
    }

    pub fn new_for_cube(min_cell: CellIndex, max_cell: CellIndex) -> Self {
        let mut chunks = HashMap::new();
        let chunk_indexes = get_required_chunks(min_cell, max_cell);
        for chunk_index in chunk_indexes {
            chunks.insert(chunk_index, Chunk::new_from_chunk_index(chunk_index));
        }
        Map {
            chunks,
            min_cell,
            max_cell,
        }
    }
    pub fn _new_from_pressures(cells: Vec<i32>, min_cell: CellIndex, max_cell: CellIndex) -> Self {
        let mut map = Self::new_for_cube(min_cell, max_cell);
        let mut i = 0;
        for cell_index in CellCubeIterator::new(min_cell, max_cell) {
            map.get_cell_mut(cell_index).pressure = cells[i];
            map.get_cell_mut(cell_index).tile_type = if cells[i] >= 0 {
                TileType::DirtyWaterWall
            } else {
                TileType::WallRock
            };
            i += 1;
        }
        map
    }

    pub fn new_from_iter(mut_map_iter: MutMapIterator) -> Self {
        Self {
            chunks: mut_map_iter.collected_chunks,
            min_cell: mut_map_iter.min_cell,
            max_cell: mut_map_iter.max_cell,
        }
    }

    pub fn default_min_cell() -> CellIndex {
        CellIndex::new(-MAP_SIZE / 2, -MAP_SIZE / 2, -MAP_SIZE / 2)
    }
    pub fn default_max_cell() -> CellIndex {
        CellIndex::new(MAP_SIZE / 2 - 1, MAP_SIZE / 2 - 1, MAP_SIZE / 2 - 1)
    }
    pub fn min_cell(&self) -> CellIndex {
        self.min_cell
    }
    pub fn max_cell(&self) -> CellIndex {
        self.max_cell
    }

    pub fn get_cell(&self, index: CellIndex) -> &Cell {
        self.get_chunk(index).get_cell(index)
    }
    pub fn get_cell_mut(&mut self, index: CellIndex) -> &mut Cell {
        self.get_chunk_mut(index).get_cell_mut(index)
    }

    fn get_chunk(&self, index: CellIndex) -> &Chunk {
        self.get_chunk_optional(&index)
            .expect("Error: Making the map bigger dynamically is disabled.")
    }

    fn get_chunk_optional(&self, index: &CellIndex) -> Option<&Chunk> {
        self.chunks.get(&get_chunk_index(*index))
    }

    #[allow(dead_code)]
    fn get_chunk_mut(&mut self, index: CellIndex) -> &mut Chunk {
        self.chunks
            .get_mut(&get_chunk_index(index))
            .expect("Error: Making the map bigger dynamically is disabled.")
    }
    pub fn regenerate(&mut self) {
        // if not provided, default seed is equal to 0
        let noise_generator = OpenSimplexNoise::new(Some(now() as i64));
        let scale = 0.2;
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
                    cell.pressure = i32::max(0, 10 - 10 * cell_index.y);
                    // cell.pressure = if tile == TileType::Air { 0 } else {40};
                }
                cell.tile_type = tile
            }
        }
        println!("simplex range used: [{}, {}]", min, max);
    }

    pub fn advance_fluids(&mut self) {
        fluids::advance_fluid(self);
    }
    pub fn _get_pressures(&self, min_cell: CellIndex, max_cell: CellIndex) -> Vec<i32> {
        let mut cells = Vec::new();
        for cell_index in CellCubeIterator::new(min_cell, max_cell) {
            cells.push(self.get_cell(cell_index).pressure);
        }
        cells
    }
    pub fn iter_mut(self) -> MutMapIterator {
        MutMapIterator::new(self.chunks, self.min_cell, self.max_cell)
    }
}

fn choose_tile(value: f64, cell_index: CellIndex) -> TileType {
    use TileType::*;
    let surface_level = trunc_towards_neg_inf((value * 0.5 * MAP_SIZE as f64) as i32, 2);
    if cell_index.y < surface_level {
        WallRock
    } else if cell_index.y > surface_level {
        if cell_index.y < 0 {
            DirtyWaterWall
        } else if cell_index.y == 0 {
            DirtyWaterSurface
        } else {
            Air
        }
    } else {
        FloorDirt
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
}
