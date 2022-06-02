const CHUNK_SIZE_X: usize = 16;
const CHUNK_SIZE_Y: usize = 4;
const CHUNK_SIZE_Z: usize = 16;
const CHUNK_SIZE: usize = CHUNK_SIZE_X * CHUNK_SIZE_Y * CHUNK_SIZE_Z;

use crate::IVec3;
use super::{Cell, CellIndex};

pub type ChunkIndex = IVec3;

pub struct Chunk {
    cells: Vec<Cell>,
}

impl Chunk {
    pub fn new() -> Self {
        let mut cells = Vec::with_capacity(CHUNK_SIZE);
        cells.resize(CHUNK_SIZE, Cell::default());
        Chunk { cells }
    }
    pub fn get_cell(&self, index: CellIndex) -> &Cell {
        self.cells.get(get_cell_index(index)).unwrap()
    }
    pub fn get_cell_mut(&mut self, index: CellIndex) -> &mut Cell {
        self.cells.get_mut(get_cell_index(index)).unwrap()
    }
}

fn get_cell_index(index: CellIndex) -> usize {
    let chunk_index = get_chunk_index(index);
    let local_index = index - origin(chunk_index);
    assert!(
        local_index.x >= 0 && local_index.x < CHUNK_SIZE_X as i32,
        "should all be in range {}",
        local_index
    );
    assert!(
        local_index.y >= 0 && local_index.y < CHUNK_SIZE_Y as i32,
        "should all be in range {}",
        local_index
    );
    assert!(
        local_index.z >= 0 && local_index.z < CHUNK_SIZE_Z as i32,
        "should all be in range {}",
        local_index
    );
    local_index.y as usize * CHUNK_SIZE_X * CHUNK_SIZE_Z
        + local_index.z as usize * CHUNK_SIZE_X
        + local_index.x as usize
}

fn origin(chunk: ChunkIndex) -> CellIndex {
    CellIndex::new(
        chunk.x * CHUNK_SIZE_X as i32,
        chunk.y * CHUNK_SIZE_Y as i32,
        chunk.z * CHUNK_SIZE_Z as i32,
    )
}

pub fn get_chunk_index(index: CellIndex) -> ChunkIndex {
    get_chunk_index_xyz(index.x, index.y, index.z)
}

pub fn get_chunk_index_xyz(x: i32, y: i32, z: i32) -> ChunkIndex {
    ChunkIndex::new(
        trunc_towards_neg_inf(x, CHUNK_SIZE_X as i32),
        trunc_towards_neg_inf(y, CHUNK_SIZE_Y as i32),
        trunc_towards_neg_inf(z, CHUNK_SIZE_Z as i32),
    )
}

fn trunc_towards_neg_inf(n: i32, chunk_size: i32) -> i32 {
    if n >= 0 {
        n / chunk_size
    } else {
        (n + 1) / chunk_size - 1
    }
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
    const H_X: i32 = CHUNK_SIZE_X as i32 / 2;
    const H_Y: i32 = CHUNK_SIZE_Y as i32 / 2;
    const H_Z: i32 = CHUNK_SIZE_Z as i32 / 2;
    const F_X: i32 = CHUNK_SIZE_X as i32;
    const F_Y: i32 = CHUNK_SIZE_Y as i32;
    const F_Z: i32 = CHUNK_SIZE_Z as i32;

    #[test]
    fn trunc() {
        assert_eq!(trunc_towards_neg_inf(0, 5), 0);
        assert_eq!(trunc_towards_neg_inf(1, 5), 0);
        assert_eq!(trunc_towards_neg_inf(2, 5), 0);
        assert_eq!(trunc_towards_neg_inf(3, 5), 0);
        assert_eq!(trunc_towards_neg_inf(4, 5), 0);
        assert_eq!(trunc_towards_neg_inf(5, 5), 1);
        assert_eq!(trunc_towards_neg_inf(-1, 5), -1);
        assert_eq!(trunc_towards_neg_inf(-2, 5), -1);
        assert_eq!(trunc_towards_neg_inf(-3, 5), -1);
        assert_eq!(trunc_towards_neg_inf(-4, 5), -1);
        assert_eq!(trunc_towards_neg_inf(-5, 5), -1);
        assert_eq!(trunc_towards_neg_inf(-6, 5), -2);
    }
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
        assert_eq!(get_cell_index(CellIndex::new(0, 0, 0)), 0);
        assert_eq!(get_cell_index(CellIndex::new(1, 0, 0)), 1);
        assert_eq!(
            get_cell_index(CellIndex::new(0, 1, 0)),
            CHUNK_SIZE_X * CHUNK_SIZE_Z
        );
        assert_eq!(get_cell_index(CellIndex::new(0, 0, 1)), CHUNK_SIZE_X);

        assert_eq!(get_cell_index(CellIndex::new(CHUNK_SIZE_X as i32, 0, 0)), 0);
        assert_eq!(get_cell_index(CellIndex::new(0, CHUNK_SIZE_Y as i32, 0)), 0);
        assert_eq!(get_cell_index(CellIndex::new(0, 0, CHUNK_SIZE_Z as i32)), 0);

        assert_eq!(
            get_cell_index(CellIndex::new(-(CHUNK_SIZE_X as i32), 0, 0)),
            0
        );
        assert_eq!(
            get_cell_index(CellIndex::new(0, -(CHUNK_SIZE_Y as i32), 0)),
            0
        );
        assert_eq!(
            get_cell_index(CellIndex::new(0, 0, -(CHUNK_SIZE_Z as i32))),
            0
        );

        assert_eq!(
            get_cell_index(CellIndex::new(1, 1, 1 - (CHUNK_SIZE_Z as i32))),
            CHUNK_SIZE_X * CHUNK_SIZE_Z + CHUNK_SIZE_X + 1
        );
    }
}