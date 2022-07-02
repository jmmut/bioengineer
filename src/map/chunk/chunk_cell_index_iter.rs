use crate::map::CellIndex;
use crate::map::chunk::{ChunkIndex, SIZE_X, SIZE_Y, SIZE_Z};

pub struct ChunkCellIndexIter {
    i: CellIndex,
    chunk_index: ChunkIndex,
}

impl ChunkCellIndexIter {
    pub fn new(chunk_index: ChunkIndex) -> Self {
        ChunkCellIndexIter {
            i: CellIndex::new(0, 0, -1),
            chunk_index,
        }
    }
}

impl Iterator for ChunkCellIndexIter {
    type Item = CellIndex;

    fn next(&mut self) -> Option<Self::Item> {
        self.i.z += 1;
        if self.i.z >= SIZE_Z as i32 {
            self.i.z = 0;
            self.i.x += 1;
        }
        if self.i.x >= SIZE_X as i32 {
            self.i.x = 0;
            self.i.y += 1;
        }
        if self.i.y >= SIZE_Y as i32 {
            Option::None
        } else {
            let absolute = chunk_local_index_to_global_index(self.i, self.chunk_index);
            Option::Some(absolute)
        }
    }
}

pub fn chunk_local_index_to_global_index(
    chunk_local_cell_index: CellIndex,
    chunk_index: ChunkIndex,
) -> CellIndex {
    let absolute = CellIndex::new(
        chunk_index.x * SIZE_X as i32 + chunk_local_cell_index.x,
        chunk_index.y * SIZE_Y as i32 + chunk_local_cell_index.y,
        chunk_index.z * SIZE_Z as i32 + chunk_local_cell_index.z,
    );
    absolute
}
