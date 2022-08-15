use crate::world::map::chunk::{get_cell_index, Chunk};
use crate::world::map::ref_mut_iterator::RefMutIterator;
use crate::world::map::{Cell, CellIndex};

pub struct CellIter {
    cells: Vec<Cell>,
    i: i32,
    origin: CellIndex,
}

impl CellIter {
    pub fn new(cells: Vec<Cell>, origin: CellIndex) -> Self {
        CellIter {
            cells,
            i: -1,
            origin,
        }
    }
    pub fn into_chunk(self) -> Chunk {
        Chunk::new_from_cells(self.cells, self.origin)
    }
    pub fn has_next(&self) -> bool {
        ((self.i + 1) as usize) < self.cells.len()
    }
}
impl Default for CellIter {
    fn default() -> Self {
        CellIter {
            cells: Vec::new(),
            i: -1,
            origin: CellIndex::default(),
        }
    }
}
pub struct CellIterItem<'a> {
    pub cell_index: CellIndex,
    pub cell: &'a mut Cell,
}

impl<'a> RefMutIterator<'a, CellIterItem<'a>> for CellIter {
    fn next(&'a mut self) -> Option<CellIterItem<'a>> {
        return if self.has_next() {
            self.i += 1;
            let i_usize = self.i as usize;
            Option::Some(CellIterItem {
                cell_index: self.origin + get_cell_index(i_usize),
                cell: &mut (self.cells[i_usize]),
            })
        } else {
            Option::None
        };
    }
}

#[cfg(test)]
mod tests {
    use super::super::SIZE;
    use super::*;

    #[test]
    fn test_cell_iter() {
        let chunk = Chunk::new(CellIndex::new(0, 0, 0));
        let mut i = 0;
        let mut iter = chunk.into_iter_mut();
        let mut sum_pressures = 0;
        while let Option::Some(item) = iter.next() {
            let CellIterItem {
                cell_index: _cell_index,
                cell,
            } = item;
            cell.pressure += 1;
            sum_pressures += cell.pressure;
            i += 1;
        }
        // iter.next();
        // while Option::Some(cell_iter_item) = iter.next() {}
        // for mut cell_iter_item in &mut iter {
        //     cell_iter_item.cell.pressure += 1;
        //     i += 1;
        // }
        let updated_chunk = iter.into_chunk();
        assert_eq!(i, SIZE);
        assert_eq!(sum_pressures, SIZE as i32);
        assert_eq!(updated_chunk.get_cell(CellIndex::new(0, 0, 0)).pressure, 1);
    }

    #[test]
    fn test_vec_iter() {
        let mut v = vec![1, 2, 3];
        let x = v.iter_mut();
        for n in x {
            *n += 10;
        }
        assert_eq!(v[0], 11);
    }
}
