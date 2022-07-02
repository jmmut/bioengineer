pub mod cell_iter;
pub mod chunk_cell_index_iter;

pub const SIZE_X: usize = 16;
pub const SIZE_Y: usize = 4;
pub const SIZE_Z: usize = 16;
const SIZE_X_I32: i32 = SIZE_X as i32;
#[allow(dead_code)]
const SIZE_Y_I32: i32 = SIZE_Y as i32;
const SIZE_Z_I32: i32 = SIZE_Z as i32;
pub const SIZE: usize = SIZE_X * SIZE_Y * SIZE_Z;

use super::trunc::trunc_towards_neg_inf;
use super::{Cell, CellIndex};
pub use crate::map::chunk::cell_iter::CellIter;
use crate::map::chunk::chunk_cell_index_iter::{
    chunk_local_index_to_global_index, ChunkCellIndexIter,
};
use crate::map::ref_mut_iterator::RefMutIterator;
use crate::IVec3;
use std::collections::HashMap;
use std::slice::Iter;

pub type ChunkIndex = IVec3;

#[derive(Clone)]
pub struct Chunk {
    cells: Vec<Cell>,
    origin: CellIndex,
}

impl Chunk {
    pub fn new(origin: CellIndex) -> Self {
        let mut cells = Vec::with_capacity(SIZE);
        cells.resize(SIZE, Cell::default());
        Chunk { cells, origin }
    }
    pub fn new_from_chunk_index(chunk_index: ChunkIndex) -> Self {
        Self::new(chunk_local_index_to_global_index(
            CellIndex::new(0, 0, 0),
            chunk_index,
        ))
    }
    pub fn new_from_cells(cells: Vec<Cell>, origin: CellIndex) -> Self {
        Self { cells, origin }
    }
    pub fn get_cell(&self, index: CellIndex) -> &Cell {
        self.cells.get(get_cell_inner_index(index)).unwrap()
    }
    pub fn get_cell_mut(&mut self, index: CellIndex) -> &mut Cell {
        self.cells.get_mut(get_cell_inner_index(index)).unwrap()
    }
    pub fn iter(&self, chunk_index: ChunkIndex) -> ChunkCellIndexIter {
        ChunkCellIndexIter::new(chunk_index)
    }

    pub fn into_iter_mut(self) -> CellIter {
        CellIter::new(self.cells, self.origin)
    }
    pub fn into_hash(self, chunks: &mut HashMap<ChunkIndex, Chunk>) {
        chunks.insert(get_chunk_index(self.origin), self);
    }
}

fn get_cell_inner_index(index: CellIndex) -> usize {
    let chunk_index = get_chunk_index(index);
    let local_index = index - origin(chunk_index);
    assert!(
        local_index.x >= 0 && local_index.x < SIZE_X as i32,
        "should all be in range {}",
        local_index
    );
    assert!(
        local_index.y >= 0 && local_index.y < SIZE_Y as i32,
        "should all be in range {}",
        local_index
    );
    assert!(
        local_index.z >= 0 && local_index.z < SIZE_Z as i32,
        "should all be in range {}",
        local_index
    );
    local_index.y as usize * SIZE_X * SIZE_Z
        + local_index.z as usize * SIZE_X
        + local_index.x as usize
}

pub fn get_cell_index(inner_index: usize) -> CellIndex {
    assert!(inner_index < SIZE_X * SIZE_Y * SIZE_Z);
    let inner_index = inner_index as i32;
    CellIndex::new(
        inner_index % SIZE_X_I32,
        inner_index / (SIZE_X_I32 * SIZE_Z_I32),
        inner_index / SIZE_X_I32 % SIZE_Z_I32,
    )
}

fn origin(chunk: ChunkIndex) -> CellIndex {
    CellIndex::new(
        chunk.x * SIZE_X as i32,
        chunk.y * SIZE_Y as i32,
        chunk.z * SIZE_Z as i32,
    )
}

pub fn get_chunk_index(index: CellIndex) -> ChunkIndex {
    get_chunk_index_xyz(index.x, index.y, index.z)
}

pub fn get_chunk_index_xyz(x: i32, y: i32, z: i32) -> ChunkIndex {
    ChunkIndex::new(
        trunc_towards_neg_inf(x, SIZE_X as i32),
        trunc_towards_neg_inf(y, SIZE_Y as i32),
        trunc_towards_neg_inf(z, SIZE_Z as i32),
    )
}

pub fn get_required_chunks(min_cell: CellIndex, max_cell: CellIndex) -> Vec<ChunkIndex> {
    let assert_less_than = |min: i32, max: i32| {
        assert!(
            min <= max,
            "failed assertion: (min_cell < max_cell). {} < {}",
            min_cell,
            max_cell
        );
    };
    assert_less_than(min_cell.x, max_cell.x);
    assert_less_than(min_cell.y, max_cell.y);
    assert_less_than(min_cell.z, max_cell.z);
    let min_chunk = get_chunk_index(min_cell);
    let max_chunk = get_chunk_index(max_cell);
    let mut chunks = Vec::new();
    for i_x in min_chunk.x..=max_chunk.x {
        for i_y in min_chunk.y..=max_chunk.y {
            for i_z in min_chunk.z..=max_chunk.z {
                chunks.push(ChunkIndex::new(i_x, i_y, i_z));
            }
        }
    }
    chunks
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::panic;

    /// half and full chunk sizes
    const H_X: i32 = SIZE_X as i32 / 2;
    const H_Y: i32 = SIZE_Y as i32 / 2;
    const H_Z: i32 = SIZE_Z as i32 / 2;
    const F_X: i32 = SIZE_X as i32;
    const F_Y: i32 = SIZE_Y as i32;
    const F_Z: i32 = SIZE_Z as i32;

    #[test]
    fn get_chunk_index_basic() {
        assert_eq!(get_chunk_index_xyz(0, 0, 0), ChunkIndex::new(0, 0, 0));
        assert_eq!(get_chunk_index_xyz(H_X, H_Y, H_Z), ChunkIndex::new(0, 0, 0));
        assert_eq!(get_chunk_index_xyz(F_X, F_Y, F_Z), ChunkIndex::new(1, 1, 1));
        assert_eq!(get_chunk_index_xyz(F_X, H_Y, H_Z), ChunkIndex::new(1, 0, 0));
    }

    #[test]
    fn get_chunk_index_positive_over_1() {
        assert_eq!(
            get_chunk_index_xyz(H_X, F_Y + H_Y, F_Z * 2 + H_Z),
            ChunkIndex::new(0, 1, 2)
        );
    }

    #[test]
    fn get_chunk_index_negative() {
        assert_eq!(
            get_chunk_index_xyz(-H_X, -H_Y, -H_Z),
            ChunkIndex::new(-1, -1, -1)
        );
        assert_eq!(
            get_chunk_index_xyz(-F_X, -F_Y, -F_Z),
            ChunkIndex::new(-1, -1, -1)
        );
    }

    #[test]
    fn get_chunk_index_negative_over_1() {
        assert_eq!(
            get_chunk_index_xyz(-H_X, -(F_Y + H_Y), -(F_Z * 2 + H_Z)),
            ChunkIndex::new(-1, -2, -3)
        );
    }

    #[test]
    fn required_chunks_basic() {
        let min_cell = CellIndex::new(0, 0, 0);
        let max_cell = CellIndex::new(0, 0, 0);
        assert_eq!(get_required_chunks(min_cell, max_cell).len(), 1);
    }

    #[test]
    fn required_chunks_positive() {
        let min_cell = CellIndex::new(0, 0, 0);
        let max_cell = CellIndex::new(F_X, 0, 2 * F_Z);
        assert_eq!(
            get_required_chunks(min_cell, max_cell),
            vec![
                ChunkIndex::new(0, 0, 0),
                ChunkIndex::new(0, 0, 1),
                ChunkIndex::new(0, 0, 2),
                ChunkIndex::new(1, 0, 0),
                ChunkIndex::new(1, 0, 1),
                ChunkIndex::new(1, 0, 2),
            ]
        );
    }

    #[test]
    fn required_chunks_negative() {
        let min_cell = CellIndex::new(-3 * F_X, -2 * F_Y, -H_Z);
        let max_cell = CellIndex::new(-H_X, -H_Y, -H_Z);
        assert_eq!(
            get_required_chunks(min_cell, max_cell),
            vec![
                ChunkIndex::new(-3, -2, -1),
                ChunkIndex::new(-3, -1, -1),
                ChunkIndex::new(-2, -2, -1),
                ChunkIndex::new(-2, -1, -1),
                ChunkIndex::new(-1, -2, -1),
                ChunkIndex::new(-1, -1, -1),
            ]
        );
    }

    #[test]
    fn required_chunks_over_origin() {
        let min_cell = CellIndex::new(-H_X, 0, -F_Z);
        let max_cell = CellIndex::new(H_X, H_Y, H_Z);
        assert_eq!(
            get_required_chunks(min_cell, max_cell),
            vec![
                ChunkIndex::new(-1, 0, -1),
                ChunkIndex::new(-1, 0, 0),
                ChunkIndex::new(0, 0, -1),
                ChunkIndex::new(0, 0, 0),
            ]
        );
    }

    #[test]
    fn cell_index_basic() {
        assert_eq!(get_cell_inner_index(CellIndex::new(0, 0, 0)), 0);
        assert_eq!(get_cell_inner_index(CellIndex::new(1, 0, 0)), 1);
        assert_eq!(
            get_cell_inner_index(CellIndex::new(0, 1, 0)),
            SIZE_X * SIZE_Z
        );
        assert_eq!(get_cell_inner_index(CellIndex::new(0, 0, 1)), SIZE_X);

        assert_eq!(get_cell_inner_index(CellIndex::new(SIZE_X as i32, 0, 0)), 0);
        assert_eq!(get_cell_inner_index(CellIndex::new(0, SIZE_Y as i32, 0)), 0);
        assert_eq!(get_cell_inner_index(CellIndex::new(0, 0, SIZE_Z as i32)), 0);

        assert_eq!(
            get_cell_inner_index(CellIndex::new(-(SIZE_X as i32), 0, 0)),
            0
        );
        assert_eq!(
            get_cell_inner_index(CellIndex::new(0, -(SIZE_Y as i32), 0)),
            0
        );
        assert_eq!(
            get_cell_inner_index(CellIndex::new(0, 0, -(SIZE_Z as i32))),
            0
        );

        assert_eq!(
            get_cell_inner_index(CellIndex::new(1, 1, 1 - (SIZE_Z as i32))),
            SIZE_X * SIZE_Z + SIZE_X + 1
        );
    }

    fn assert_reverse_index(n: usize) {
        assert_eq!(get_cell_inner_index(get_cell_index(n)), n);
    }

    #[test]
    fn test_cell_reverse_index() {
        assert_reverse_index(0);
        assert_reverse_index(1);
        assert_reverse_index(SIZE_X);
        assert_reverse_index(SIZE_X + 1);
        assert_reverse_index(SIZE_Y);
        assert_reverse_index(SIZE_Y + 1);
        assert_reverse_index(SIZE_Z);
        assert_reverse_index(SIZE_Z + 1);
        assert_reverse_index(SIZE_X * SIZE_Z);
        assert_reverse_index(SIZE_X * SIZE_Z + 1);
        assert_reverse_index(SIZE_X * SIZE_Y * SIZE_Z - 1);
    }

    fn assert_throws(f: fn()) {
        assert!(panic::catch_unwind(f).is_err());
    }

    #[test]
    fn test_cell_reverse_index_panics() {
        assert_throws(|| {
            get_cell_index(SIZE_X * SIZE_Y * SIZE_Z);
        });
    }
}
