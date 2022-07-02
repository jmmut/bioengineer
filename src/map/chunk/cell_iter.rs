use crate::map::{Cell, CellIndex};
use crate::map::chunk::{Chunk, get_cell_index};
use crate::map::ref_mut_iterator::RefMutIterator;

pub struct CellIter {
    cells: Vec<Cell>,
    i: i32,
    origin: CellIndex,
}

impl CellIter {

    pub fn new(cells: Vec<Cell>, origin: CellIndex) -> Self {
        CellIter { cells, i: -1, origin }
    }
    fn into_chunk(self) -> Chunk {
        Chunk::new_from_cells(self.cells, self.origin)
    }
}
pub struct CellIterItem<'a> {
    cell_index: CellIndex,
    cell: &'a mut Cell,
}


impl<'a> RefMutIterator<'a, CellIterItem<'a>> for &mut CellIter {
    fn next(&'a mut self) -> Option<CellIterItem<'a>> {
        self.i += 1;
        let i_usize = self.i as usize;
        return if i_usize < self.cells.len() {
            Option::Some(CellIterItem {
                cell_index: self.origin + get_cell_index(i_usize),
                cell: &mut (self.cells[i_usize])}
            )
        } else {
            Option::None
        };
    }

}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::SIZE;

    #[test]
    fn test_cell_iter() {
        let chunk = Chunk::new(CellIndex::new(0, 0, 0));
        let mut i = 0;
        let mut iter = chunk.into_iter();
        let mut sum_pressures = 0;
        while let Option::Some(CellIterItem{cell_index, cell}) = (&mut iter).next() {
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
