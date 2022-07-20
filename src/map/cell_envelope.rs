use crate::map::CellIndex;

fn envelope(cells: &Vec<CellIndex>) -> (CellIndex, CellIndex) {
    let Envelope {
        mut min_cell,
        mut max_cell,
    } = Envelope::new();
    for cell_index in cells {
        if cell_index.x < min_cell.x {
            min_cell.x = cell_index.x
        }
        if cell_index.y < min_cell.y {
            min_cell.y = cell_index.y
        }
        if cell_index.z < min_cell.z {
            min_cell.z = cell_index.z
        }
        if cell_index.x > max_cell.x {
            max_cell.x = cell_index.x
        }
        if cell_index.y > max_cell.y {
            max_cell.y = cell_index.y
        }
        if cell_index.z > max_cell.z {
            max_cell.z = cell_index.z
        }
    }
    (min_cell, max_cell)
}

struct Envelope {
    min_cell: CellIndex,
    max_cell: CellIndex,
}

impl Envelope {
    pub fn new() -> Self {
        let mut min_cell = CellIndex::new(i32::MAX, i32::MAX, i32::MAX);
        let mut max_cell = CellIndex::new(i32::MIN, i32::MIN, i32::MIN);
        Envelope { min_cell, max_cell }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_envelope_basic() {
        let some_pos = CellIndex::new(10, 0, -20);
        let cells = vec![some_pos];
        assert_eq!(envelope(&cells), (some_pos, some_pos));
    }

    #[test]
    fn test_envelope() {
        let some_pos = CellIndex::new(10, 0, -20);
        let other_pos = CellIndex::new(-10, 5, 20);
        let cells = vec![some_pos, other_pos];
        assert_eq!(
            envelope(&cells),
            (CellIndex::new(-10, 0, -20), CellIndex::new(10, 5, 20))
        );
    }

    #[test]
    fn test_envelope_middle_cells() {
        let cells = vec![
            CellIndex::new(10, 0, -20),
            CellIndex::new(2, 50, -4),
            CellIndex::new(-10, 5, 20),
        ];
        assert_eq!(
            envelope(&cells),
            (CellIndex::new(-10, 0, -20), CellIndex::new(10, 50, 20))
        );
    }
}
