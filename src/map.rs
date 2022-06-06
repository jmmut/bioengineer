mod chunk;
pub mod trunc;

use crate::map::chunk::{get_chunk_index, get_required_chunks};
use crate::map::TileType::{Air, FloorDirt, WallRock};
use crate::{now, IVec3};
use chunk::{Chunk, ChunkIndex};
use fluent_asserter::prelude::*;
use opensimplex_noise_rs::OpenSimplexNoise;
use std::cmp::{max, min};
use std::collections::HashMap;
pub use trunc::trunc_towards_neg_inf;

/// The axis are isometric:
/// - x: right towards camera
/// - y: up
/// - z: left towards camera
pub type CellIndex = IVec3;

const MAP_SIZE: i32 = 64;

pub struct Map {
    chunks: HashMap<ChunkIndex, Chunk>,
}

impl Map {
    pub fn new() -> Self {
        let mut chunks = HashMap::new();
        let chunk_indexes = get_required_chunks(Self::min_cell(), Self::max_cell());
        for chunk_index in chunk_indexes {
            chunks.insert(chunk_index, Chunk::new());
        }
        Map { chunks }
    }

    pub fn min_cell() -> CellIndex {
        CellIndex::new(-MAP_SIZE / 2, -MAP_SIZE / 2, -MAP_SIZE / 2)
    }
    pub fn max_cell() -> CellIndex {
        CellIndex::new(MAP_SIZE / 2 - 1, MAP_SIZE / 2 - 1, MAP_SIZE / 2 - 1)
    }

    pub fn get_cell(&self, index: CellIndex) -> &Cell {
        self.get_chunk(index).get_cell(index)
    }
    pub fn _get_cell_mut(&mut self, index: CellIndex) -> &mut Cell {
        self.get_chunk_mut(index).get_cell_mut(index)
    }

    fn get_chunk(&self, index: CellIndex) -> &Chunk {
        self.chunks
            .get(&get_chunk_index(index))
            .expect("Error: Making the map bigger dynamically is disabled.")
    }
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
                chunk.get_cell_mut(cell_index).tile_type = choose_tile(value, cell_index)
            }
        }
        println!("simplex range used: [{}, {}]", min, max);
    }
}

fn choose_tile(value: f64, cell_index: CellIndex) -> TileType {
    let surface_level = trunc_towards_neg_inf((value * 0.5 * MAP_SIZE as f64) as i32, 2);
    if surface_level > cell_index.y {
        WallRock
    } else if surface_level < cell_index.y {
        Air
    } else {
        FloorDirt
    }
}

#[derive(Clone, Copy)]
pub struct Cell {
    pub tile_type: TileType,
}

#[derive(Clone, Copy)]
pub enum TileType {
    Unset = -1,
    // Helper = 2,
    WallRock = 16,
    WallDirt = 24,
    FloorRock = 17,
    FloorDirt = 20,
    Air = 29,
    MachineAssembler = 12,
    MachineDrill = 13,
    MachineSolarPanel = 21,
    MachineShip = 28,
}

impl Default for Cell {
    fn default() -> Self {
        Cell {
            tile_type: TileType::WallRock,
        }
    }
}

pub struct CellCubeIterator {
    min_cell: CellIndex,
    max_cell: CellIndex,
    cursor: CellIndex,
}

impl CellCubeIterator {
    /// Inclusive range. min_cell and max_cell are both included in the loop.
    pub fn new(min_cell: CellIndex, max_cell: CellIndex) -> Self {
        assert!(
            min_cell.x <= max_cell.x && min_cell.y <= max_cell.y && min_cell.z <= max_cell.z,
            "min_cell has to be smaller or equal than max_cell in all the components: {} <= {}",
            min_cell,
            max_cell
        );
        let mut cursor = min_cell;
        cursor.x -= 1;
        Self {
            min_cell,
            max_cell,
            cursor,
        }
    }

    /// Use this to not bother about splitting into minimum and maximum coordinates.
    /// Inclusive range. cell and other_cell are both included in the loop.
    pub fn new_from_mixed(cell: CellIndex, other_cell: CellIndex) -> Self {
        let min_cell = CellIndex::new(
            min(cell.x, other_cell.x),
            min(cell.y, other_cell.y),
            min(cell.z, other_cell.z),
        );
        let max_cell = CellIndex::new(
            max(cell.x, other_cell.x),
            max(cell.y, other_cell.y),
            max(cell.z, other_cell.z),
        );
        Self::new(min_cell, max_cell)
    }
}

impl Iterator for CellCubeIterator {
    type Item = CellIndex;

    fn next(&mut self) -> Option<Self::Item> {
        self.cursor.x += 1;
        if self.cursor.x > self.max_cell.x {
            self.cursor.x = self.min_cell.x;
            self.cursor.z += 1;
        }
        if self.cursor.z > self.max_cell.z {
            self.cursor.z = self.min_cell.z;
            self.cursor.y += 1;
        }
        if self.cursor.y > self.max_cell.y {
            Option::None
        } else {
            Option::Some(self.cursor)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use fluent_asserter::assert_that_code;

    #[test]
    fn test_iterate_cells() {
        let mut indexes = Vec::new();
        let min_cell = CellIndex::new(-1, 0, 10);
        let max_cell = CellIndex::new(0, 1, 11);
        let cell_cube = CellCubeIterator::new(min_cell, max_cell);
        for cell_index in cell_cube {
            indexes.push(cell_index);
        }
        assert_eq!(
            indexes,
            vec![
                CellIndex::new(-1, 0, 10),
                CellIndex::new(0, 0, 10),
                CellIndex::new(-1, 0, 11),
                CellIndex::new(0, 0, 11),
                CellIndex::new(-1, 1, 10),
                CellIndex::new(0, 1, 10),
                CellIndex::new(-1, 1, 11),
                CellIndex::new(0, 1, 11),
            ]
        )
    }

    #[test]
    fn test_iterate_cells_panics() {
        let min_cell = CellIndex::new(0, 0, 0);
        let max_cell = CellIndex::new(0, -1, 0);
        assert_that_code!(|| CellCubeIterator::new(min_cell, max_cell)).panics();
    }
    #[test]
    fn test_iterate_single_cell() {
        let min_cell = CellIndex::new(0, 0, 0);
        let max_cell = CellIndex::new(0, 0, 0);
        let cell_cube = CellCubeIterator::new(min_cell, max_cell);

        let mut indexes = Vec::new();
        for cell_index in cell_cube {
            indexes.push(cell_index);
        }
        assert_eq!(indexes, vec![CellIndex::new(0, 0, 0)]);
    }
}
