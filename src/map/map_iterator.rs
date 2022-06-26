use std::collections::hash_map::Iter;
use std::collections::HashMap;
use crate::map::{Cell, Map};
use crate::map::chunk::{Chunk, ChunkIndex, ChunkCellIndexIter, CellIter};

pub struct MapIterator<'a> {
    chunks: &'a HashMap<ChunkIndex, Chunk>,
    chunk_hash_iter: Iter<'a, ChunkIndex, Chunk>,
    chunk_cell_iter: Option<CellIter<'a>>,
}

impl MapIterator<'_> {
    pub fn new<'a>(chunks: &'a HashMap<ChunkIndex, Chunk>) -> MapIterator::<'a> {
        let mut chunk_hash_iter = chunks.iter();
        let chunk_cell_iter = chunk_hash_iter.next().map(|t| {
            t.1.iter_cell()
        });
        MapIterator::<'a> { chunks, chunk_hash_iter, chunk_cell_iter }
    }
}

impl Iterator for MapIterator<'_> {
    type Item = Cell;

    fn next(&mut self) -> Option<Self::Item> {
        match &self.chunk_cell_iter {
            None => {
                self.chunk_cell_iter = self.chunk_hash_iter.next().map(|t| {
                    t.1.iter_cell()
                });
                self.chunk_cell_iter.and_then(|mut it :CellIter| {
                    it.next()
                })
            }
            Some(mut iter) => {
                iter.next()
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::map::{CellIndex, chunk, Map};

    #[test]
    fn basic_map_iterator() {
        let map = Map::new_for_cube(CellIndex::new(0, 0, 0), CellIndex::new(0, 0, 1));
        let mut i = 0;
        for cell in map.iter() {
            i += 1;
        }
        assert_eq!(i, chunk::SIZE);
    }
}
