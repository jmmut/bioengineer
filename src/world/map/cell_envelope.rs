use crate::world::map::CellIndex;

#[allow(unused)]
fn envelope(cells: &[CellIndex]) -> (CellIndex, CellIndex) {
    let mut envelope = Envelope::new();
    for cell_index in cells {
        envelope.add(cell_index);
    }
    envelope.result()
}

#[allow(unused)]
pub struct Envelope {
    pub min_cell: CellIndex,
    pub max_cell: CellIndex,
}

#[allow(unused)]
impl Envelope {
    pub fn new() -> Self {
        let min_cell = CellIndex::new(i32::MAX, i32::MAX, i32::MAX);
        let max_cell = CellIndex::new(i32::MIN, i32::MIN, i32::MIN);
        Envelope { min_cell, max_cell }
    }

    pub fn add(&mut self, cell_index: &CellIndex) {
        if cell_index.x < self.min_cell.x {
            self.min_cell.x = cell_index.x
        }
        if cell_index.y < self.min_cell.y {
            self.min_cell.y = cell_index.y
        }
        if cell_index.z < self.min_cell.z {
            self.min_cell.z = cell_index.z
        }
        if cell_index.x > self.max_cell.x {
            self.max_cell.x = cell_index.x
        }
        if cell_index.y > self.max_cell.y {
            self.max_cell.y = cell_index.y
        }
        if cell_index.z > self.max_cell.z {
            self.max_cell.z = cell_index.z
        }
    }

    pub fn result(&self) -> (CellIndex, CellIndex) {
        (self.min_cell, self.max_cell)
    }
}

#[allow(unused)]
pub fn is_inside(cell_index: &CellIndex, envelope: &Envelope) -> bool {
    cell_index.x >= envelope.min_cell.x
        && cell_index.x <= envelope.max_cell.x
        && cell_index.y >= envelope.min_cell.y
        && cell_index.y <= envelope.max_cell.y
        && cell_index.z >= envelope.min_cell.z
        && cell_index.z <= envelope.max_cell.z
}

pub fn is_horizontally_inside(cell_index: &CellIndex, envelope: &Envelope) -> bool {
    cell_index.x >= envelope.min_cell.x
        && cell_index.x <= envelope.max_cell.x
        && cell_index.z >= envelope.min_cell.z
        && cell_index.z <= envelope.max_cell.z
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

    #[test]
    fn test_envelope_iter() {
        let cells = vec![
            CellIndex::new(10, 0, -20),
            CellIndex::new(2, 50, -4),
            CellIndex::new(-10, 5, 20),
        ];
        let mut envelope = Envelope::new();
        for cell_index in cells {
            envelope.add(&cell_index);
        }
        assert_eq!(
            envelope.result(),
            (CellIndex::new(-10, 0, -20), CellIndex::new(10, 50, 20))
        );
    }

    #[test]
    fn test_is_inside() {
        let envelope = Envelope {
            min_cell: CellIndex::new(3, 4, 5),
            max_cell: CellIndex::new(10, 8, 6),
        };
        assert_eq!(is_inside(&CellIndex::new(5, 4, 6), &envelope), true);
        assert_eq!(is_inside(&CellIndex::new(2, 4, 6), &envelope), false);
    }
}
