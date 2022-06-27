pub const SIZE_X: usize = 16;
pub const SIZE_Y: usize = 4;
pub const SIZE_Z: usize = 16;
pub const SIZE: usize = SIZE_X * SIZE_Y * SIZE_Z;

use super::trunc::trunc_towards_neg_inf;
use super::{Cell, CellIndex};
use crate::IVec3;
use std::slice::Iter;

pub type ChunkIndex = IVec3;

pub struct Chunk {
    cells: Vec<Cell>,
}

impl Chunk {
    pub fn new() -> Self {
        let mut cells = Vec::with_capacity(SIZE);
        cells.resize(SIZE, Cell::default());
        Chunk { cells }
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
}

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
            let absolute = CellIndex::new(
                self.chunk_index.x * SIZE_X as i32 + self.i.x,
                self.chunk_index.y * SIZE_Y as i32 + self.i.y,
                self.chunk_index.z * SIZE_Z as i32 + self.i.z,
            );
            Option::Some(absolute)
        }
    }
}

pub enum CellIter<'a> {
    Inner(Iter<'a, Cell>),
    Empty,
}

impl<'a> IntoIterator for &'a Chunk {
    type Item = Cell;
    type IntoIter = CellIter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        let iter = self.cells.iter();
        Self::IntoIter::Inner(iter)
    }
}

impl Iterator for CellIter<'_> {
    type Item = Cell;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            CellIter::Inner(iter) => iter.next().map(|c| *c),
            CellIter::Empty => Option::None,
        }
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

    #[test]
    fn test_cell_iter() {
        let chunk = Chunk::new();
        let mut sum_pressure = 0;
        let mut i = 0;
        for cell in &chunk {
            sum_pressure += cell.pressure;
            i += 1;
        }
        assert_eq!(i, SIZE);
    }
}
