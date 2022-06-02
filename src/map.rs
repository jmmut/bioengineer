mod chunk;

use crate::map::chunk::{get_chunk_index, get_required_chunks};
use chunk::{Chunk, ChunkIndex};
use crate::IVec3;
use std::collections::HashMap;

/// The axis are isometric:
/// - x: right towards camera
/// - y: up
/// - z: left towards camera
pub type CellIndex = IVec3;

const MAP_SIZE: i32 = 32;

impl Map {
    pub fn new() -> Self {
        let mut chunks = HashMap::new();
        let chunk_indexes = get_required_chunks(
            CellIndex::new(0, 0, 0),
            CellIndex::new(MAP_SIZE - 1, MAP_SIZE - 1, MAP_SIZE - 1),
        );
        for chunk_index in chunk_indexes {
            chunks.insert(chunk_index, Chunk::new());
        }
        Map { chunks }
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
}

pub struct Map {
    chunks: HashMap<ChunkIndex, Chunk>,
}

#[derive(Clone, Copy)]
pub struct Cell {
    pub tile_type: TileType,
}

#[derive(Clone, Copy)]
pub enum TileType {
    Unset,
    WallRock = 17,
    WallDirt = 25,
    FloorRock = 18,
    FloorDirt = 21,
    Air = 27,
    MachineAssembler = 13,
    MachineDrill = 14,
    MachineSolarPanel = 22,
    MachineShip = 29,
}

impl Default for Cell {
    fn default() -> Self {
        Cell {
            tile_type: TileType::WallRock,
        }
    }
}
