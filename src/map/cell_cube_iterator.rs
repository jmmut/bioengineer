use crate::map::CellIndex;
use std::cmp::{max, min};

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
    use fluent_asserter::*;

    use super::*;

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
