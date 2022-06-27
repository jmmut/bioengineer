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
    chunk_iterator: Iter<'a, ChunkIndex, Chunk>,
    cell_iterator: Option<CellIter<'a>>,
}

impl<'a> IntoIterator for &'a Map {
    type Item = Cell;
    type IntoIter = MapIterator<'a>;

    fn into_iter(self) -> Self::IntoIter {
        let mut chunk_iterator = self.chunks.iter();
        let cell_iterator = chunk_iterator.next().map(|chunk| chunk.1.into_iter());
        MapIterator::<'a> {
            chunks: &self.chunks,
            chunk_iterator,
            cell_iterator,
        }
    }
}

impl Iterator for MapIterator<'_> {
    type Item = Cell;

    fn next(&mut self) -> Option<Self::Item> {
        match &mut self.cell_iterator {
            None => Option::None,
            Some(_) => {
                let cell = self.cell_iterator.as_mut().unwrap().next();
                cell.or_else(|| {
                    self.cell_iterator =
                        (&mut self.chunk_iterator.next()).map(|chunk| chunk.1.into_iter());
                    let cell = self
                        .cell_iterator
                        .as_mut()
                        .and_then(|ci: &mut CellIter| ci.next());
                    cell
                })
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::map::{chunk, Cell, CellIndex, Map};

    #[test]
    fn test_basic_map_iterator() {
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
