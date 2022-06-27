use crate::map::chunk::{CellIter, Chunk, ChunkCellIndexIter, ChunkIndex};
use crate::map::{Cell, Map};
use std::collections::hash_map::Iter;
use std::collections::HashMap;

/// Note that this iterator needs a &Map. That is, iterate a map by reference:
/// ```rust
///     let map = Map::new_for_cube(
///         CellIndex::new(0, 0, 0),
///         CellIndex::new(0, 0, chunk::SIZE_X as i32 + 1),
///     );
///     let mut i = 0;
///     let mut sum_pressure = 0;
///     for cell in &map {
///         sum_pressure += cell.pressure;
///         i += 1;
///     }
///     assert_eq!(i, 2*chunk::SIZE);
/// ```
pub struct MapIterator<'a> {
    chunks: &'a HashMap<ChunkIndex, Chunk>,
    chunk_hash_iter: Iter<'a, ChunkIndex, Chunk>,
    chunk_cell_iter: Option<CellIter<'a>>,
}

impl<'a> IntoIterator for &'a Map {
    type Item = Cell;
    type IntoIter = MapIterator<'a>;

    fn into_iter(self) -> Self::IntoIter {
        let mut chunk_hash_iter = self.chunks.iter();
        let chunk_cell_iter = chunk_hash_iter.next().map(|t| t.1.into_iter());
        MapIterator::<'a> {
            chunks: &self.chunks,
            chunk_hash_iter,
            chunk_cell_iter,
        }
    }
}

impl Iterator for MapIterator<'_> {
    type Item = Cell;

    fn next(&mut self) -> Option<Self::Item> {
        match &mut self.chunk_cell_iter {
            None => Option::None,
            Some(_) => {
                let cell = self.chunk_cell_iter.as_mut().unwrap().next();
                return if cell.is_none() {
                    let chunk_option = &mut self.chunk_hash_iter.next();
                    if chunk_option.is_none() {
                        Option::None
                    } else {
                        self.chunk_cell_iter = chunk_option.map(|t| t.1.into_iter());
                        let cell = self.chunk_cell_iter.as_mut().unwrap().next();
                        cell
                    }
                } else {
                    cell
                };
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::map::{chunk, Cell, CellIndex, Map};

    #[test]
    fn basic_map_iterator() {
        let map = Map::new_for_cube(CellIndex::new(0, 0, 0), CellIndex::new(0, 0, 1));
        let mut i = 0;
        let mut sum_pressure = 0;
        for cell in map.into_iter() {
            sum_pressure += cell.pressure;
            i += 1;
        }
        assert_eq!(i, chunk::SIZE);
    }

    #[test]
    fn test_several_chunk_map_iterator() {
        let map = Map::new_for_cube(
            CellIndex::new(0, 0, 0),
            CellIndex::new(0, 0, chunk::SIZE_X as i32 + 1),
        );
        let mut i = 0;
        let mut sum_pressure = 0;
        for cell in &map {
            sum_pressure += cell.pressure;
            i += 1;
        }
        assert_eq!(i, 2 * chunk::SIZE);
    }
}
