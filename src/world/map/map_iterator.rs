use crate::world::map::chunk::cell_iter::CellIterItem;
use crate::world::map::chunk::{CellIter, Chunk, ChunkIndex, chunks};
use crate::world::map::ref_mut_iterator::RefMutIterator;
use crate::world::map::CellIndex;
use std::collections::hash_map::IntoIter;
use std::collections::HashMap;
use crate::world::map::chunk::chunks::Chunks;

/*
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
    cell_iterator: CellIter,
}

impl<'a> IntoIterator for &'a Map {
    type Item = (CellIndex, Cell);
    type IntoIter = MapIterator<'a>;

    fn into_iter(self) -> Self::IntoIter {
        let mut chunk_iterator = self.chunks.iter();
        let optional_chunk = chunk_iterator.next();
        let cell_iterator = optional_chunk
            .map(|chunk| chunk.1.into_iter())
            .unwrap_or(CellIter::Empty);
        MapIterator::<'a> {
            chunks: &self.chunks,
            chunk_iterator,
            cell_iterator,
        }
    }
}

impl Iterator for MapIterator<'_> {
    type Item = (CellIndex, Cell);

    fn next(&mut self) -> Option<Self::Item> {
        let cell = self.cell_iterator.next();
        cell.or_else(|| {
            let optional_chunk = &mut self.chunk_iterator.next();
            self.cell_iterator = optional_chunk
                .map(|chunk| chunk.1.into_iter())
                .unwrap_or(CellIter::Empty);
            let cell = self.cell_iterator.next();
            cell
        })
    }
}

*/

pub struct MutMapIterator {
    chunk_iterator: chunks::IntoIter,
    cell_iterator: CellIter,
    pub collected_chunks: Chunks,
    pub min_cell: CellIndex,
    pub max_cell: CellIndex,
    pub ship_position: Option<CellIndex>,
}

impl MutMapIterator {
    pub fn new(
        chunks: Chunks,
        min_cell: CellIndex,
        max_cell: CellIndex,
        ship_position: Option<CellIndex>,
    ) -> Self {
        let mut chunk_iterator = chunks.into_iter();
        let optional_chunk = chunk_iterator.next();
        let cell_iterator = optional_chunk
            .map(|chunk| chunk.1.into_iter_mut())
            .unwrap_or_default();
        Self {
            chunk_iterator,
            cell_iterator,
            collected_chunks: Chunks::new(),
            min_cell,
            max_cell,
            ship_position,
        }
    }

    fn advance_to_next_chunk(&mut self) -> Option<CellIterItem> {
        let optional_next_chunk = self.chunk_iterator.next();
        let next_chunk_cell_iterator = optional_next_chunk
            .map(|chunk| chunk.1.into_iter_mut())
            .unwrap_or_default();
        let previous_chunk_cell_iterator =
            std::mem::replace(&mut self.cell_iterator, next_chunk_cell_iterator);
        previous_chunk_cell_iterator
            .into_chunk()
            .into_hash(&mut self.collected_chunks);
        self.cell_iterator.next()
    }
}

impl<'a> RefMutIterator<'a, CellIterItem<'a>> for MutMapIterator {
    fn next(&'a mut self) -> Option<CellIterItem<'a>> {
        if self.cell_iterator.has_next() {
            self.cell_iterator.next()
        } else {
            self.advance_to_next_chunk()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::world::map::chunk;
    use crate::world::map::Map;
    // #[test]
    // fn test_basic_map_iterator() {
    //     let map = Map::new_for_cube(CellIndex::new(0, 0, 0), CellIndex::new(0, 0, 1));
    //     let mut i = 0;
    //     let mut sum_pressure = 0;
    //     for (cell_index, cell) in map.into_iter() {
    //         sum_pressure += cell.pressure;
    //         i += 1;
    //     }
    //     assert_eq!(i, chunk::SIZE);
    // }

    // #[test]
    // fn test_several_chunk_map_iterator() {
    //     let map = Map::new_for_cube(
    //         CellIndex::new(0, 0, 0),
    //         CellIndex::new(0, 0, chunk::SIZE_X as i32 + 1),
    //     );
    //     let mut i = 0;
    //     let mut sum_pressure = 0;
    //     for (cell_index, cell) in &map {
    //         sum_pressure += cell.pressure;
    //         i += 1;
    //     }
    //     assert_eq!(i, 2 * chunk::SIZE);
    // }

    #[test]
    fn test_basic_mut_map_iterator() {
        let map = Map::new_for_cube(CellIndex::new(0, 0, 0), CellIndex::new(0, 0, 1));
        let mut i = 0;
        let mut sum_pressure = 0;
        let mut iter = map.iter_mut();
        while let Option::Some(item) = iter.next() {
            let CellIterItem {
                cell_index: _cell_index,
                cell,
            } = item;
            cell.pressure += 10;
            sum_pressure += cell.pressure;
            i += 1;
        }
        let updated_map = Map::new_from_iter(iter);
        assert_eq!(i, chunk::SIZE);
        assert_eq!(sum_pressure, 10 * chunk::SIZE as i32);
        assert_eq!(updated_map.get_cell(CellIndex::new(0, 0, 0)).pressure, 10)
    }
}
