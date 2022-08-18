
// comment/uncomment to choose Chunks implementation:
// pub use hash_impl::{Chunks, IntoIter};
pub use vec_impl::{Chunks, IntoIter};
// pub type Chunks = HashMap<ChunkIndex, Chunk>;
// pub type IntoIter = std::collections::hash_map::IntoIter<ChunkIndex, Chunk>;

pub mod cache;

mod vec_impl {
    use std::collections::HashMap;
    use crate::world::map::chunk::{Chunk, ChunkIndex};
    use crate::world::map::chunk::chunks::cache::{IndexCache, record_cache_cold_hit, record_cache_hot_hit, record_cache_miss};

    pub type IntoIter = IntoIter2;


    pub struct Chunks {
        inner_vec: Vec<(ChunkIndex, Chunk)>,
        inner_map: HashMap<ChunkIndex, usize>,
        recently_used: IndexCache,
    }

    impl Chunks {
        pub fn new() -> Self {
            Chunks {
                inner_vec: Vec::new(),
                inner_map: HashMap::new(),
                recently_used: IndexCache::new(),
            }
        }

        pub fn insert(&mut self, chunk_index: ChunkIndex, chunk: Chunk) -> Option<Chunk> {
            let existing_opt = self.get_mut(&chunk_index);
            return if let Option::Some(existing) = existing_opt {
                let previous = existing.clone();
                *existing = chunk;
                Option::Some(previous)
            } else {
                self.inner_vec.push((chunk_index, chunk));
                self.inner_map.insert(chunk_index, self.inner_vec.len() - 1);
                Option::None
            }
        }

        /// note that get_mut doesn't use the cache! Didn't need it so far
        pub fn get_mut(&mut self, chunk_index: &ChunkIndex) -> Option<&mut Chunk> {
            if let Option::Some(i) = self.inner_map.get(chunk_index) {
                return self.inner_vec.get_mut(*i).map(|entry| &mut entry.1)
            }
            return Option::None
        }

        pub fn get(&self, chunk_index: &ChunkIndex) -> Option<&Chunk> {
            let result = self.try_hot_cache(chunk_index);
            if result.is_some() {
                return result;
            }
            let result = self.try_cold_cache(chunk_index);
            if result.is_some() {
                return result;
            }
            record_cache_miss();
            self.full_scan(chunk_index)
        }

        fn full_scan(&self, chunk_index: &ChunkIndex) -> Option<&Chunk> {
            if let Option::Some(i) = self.inner_map.get(chunk_index) {
                self.recently_used.add_to_cache(*i);
                return self.inner_vec.get(*i).map(|entry| &entry.1)
            }
            return Option::None
        }

        fn try_hot_cache(&self, chunk_index: &ChunkIndex) -> Option<&Chunk> {
            let i = self.recently_used.get_hot_cached_index();
            if let Option::Some(entry) = self.inner_vec.get(i) {
                if entry.0.eq(chunk_index) {
                    record_cache_hot_hit();
                    return Option::Some(&entry.1)
                }
            }
            Option::None
        }

        fn try_cold_cache(&self, chunk_index: &ChunkIndex) -> Option<&Chunk> {
            let i = self.recently_used.get_cold_cached_index();
            if let Option::Some(entry) = self.inner_vec.get(i) {
                if entry.0.eq(chunk_index) {
                    self.recently_used.add_to_cache(i);
                    record_cache_cold_hit();
                    return Option::Some(&entry.1)
                }
            }
            Option::None
        }

        pub fn len(&self) -> usize {
            self.inner_vec.len()
        }

        pub fn iter(&self) -> Iter2<'_> {
            Iter2 { inner: self.inner_vec.iter() }
        }

        pub fn iter_mut(&mut self) -> IterMut2<'_> {
            IterMut2 {inner: self.inner_vec.iter_mut() }
        }
    }
    impl Clone for Chunks {
        fn clone(&self) -> Self {
            Chunks {
                inner_vec: self.inner_vec.clone(),
                inner_map: self.inner_map.clone(),
                recently_used: IndexCache::new(),
            }
        }
    }

    impl IntoIterator for Chunks {
        type Item = (ChunkIndex, Chunk);
        type IntoIter = IntoIter2;

        fn into_iter(self) -> Self::IntoIter {
            IntoIter2(self.inner_vec.into_iter())
        }
    }

    impl<'a> IntoIterator for &'a mut Chunks {
        type Item = &'a mut (ChunkIndex, Chunk);
        type IntoIter = IterMut2<'a>;

        fn into_iter(self) -> Self::IntoIter {
            IterMut2{ inner: self.inner_vec.iter_mut() }
        }
    }

    pub struct IntoIter2(std::vec::IntoIter<(ChunkIndex, Chunk)>);

    impl Iterator for IntoIter2 {
        type Item = (ChunkIndex, Chunk);
        fn next(&mut self) -> Option<Self::Item> {
            self.0.next()
        }
    }


    pub struct Iter2<'a> {
        inner: std::slice::Iter<'a, (ChunkIndex, Chunk)>,
    }

    impl<'a> Iterator for Iter2<'a> {
        type Item = &'a(ChunkIndex, Chunk);
        fn next(&mut self) -> Option<Self::Item> {
            self.inner.next()
        }
    }


    pub struct IterMut2<'a> {
        inner: std::slice::IterMut<'a, (ChunkIndex, Chunk)>,
    }

    impl<'a> Iterator for IterMut2<'a> {
        type Item =  &'a mut (ChunkIndex, Chunk);

        fn next(&mut self) -> Option<Self::Item> {
            self.inner.next()
        }
    }

}

mod hash_impl {
    use std::collections::HashMap;
    use crate::world::map::chunk::{Chunk, ChunkIndex};
    // pub type Chunks = HashMap<ChunkIndex, Chunk>;
    #[allow(unused)]
    pub type IntoIter = IntoIter2;

    pub struct Chunks {
        inner_map: HashMap<ChunkIndex, Chunk>,
    }

    impl Chunks {
        pub fn new() -> Self {
            Chunks {
                inner_map : HashMap::new()
            }
        }

        pub fn insert(&mut self, chunk_index: ChunkIndex, chunk: Chunk) -> Option<Chunk> {
            self.inner_map.insert(chunk_index, chunk)
        }

        pub fn get_mut(&mut self, chunk_index: &ChunkIndex) -> Option<&mut Chunk>{
            self.inner_map.get_mut(chunk_index)
        }

        pub fn get(&self, chunk_index: &ChunkIndex) -> Option<&Chunk> {
            self.inner_map.get(chunk_index)
        }

        pub fn len(&self) -> usize {
            self.inner_map.len()
        }

        pub fn iter(&self) -> Iter2<'_> {
            Iter2 { inner: self.inner_map.iter() }
        }

        pub fn iter_mut(&mut self) -> IterMut2<'_> {
            IterMut2 {inner: self.inner_map.iter_mut() }
        }
    }
    impl Clone for Chunks {
        fn clone(&self) -> Self {
            Chunks {inner_map: self.inner_map.clone() }
        }
    }

    impl IntoIterator for Chunks {
        type Item = (ChunkIndex, Chunk);
        type IntoIter = IntoIter2;

        fn into_iter(self) -> Self::IntoIter {
            IntoIter2(self.inner_map.into_iter())
        }
    }

    impl<'a> IntoIterator for &'a mut Chunks {
        type Item = (&'a ChunkIndex, &'a mut Chunk);
        type IntoIter = IterMut2<'a>;

        fn into_iter(self) -> Self::IntoIter {
            IterMut2{ inner: self.inner_map.iter_mut() }
        }
    }

    pub struct IntoIter2(std::collections::hash_map::IntoIter<ChunkIndex, Chunk>);

    impl Iterator for IntoIter2 {
        type Item = (ChunkIndex, Chunk);
        fn next(&mut self) -> Option<Self::Item> {
            self.0.next()
        }
    }


    pub struct Iter2<'a> {
        inner: std::collections::hash_map::Iter<'a, ChunkIndex, Chunk>,
    }

    impl<'a> Iterator for Iter2<'a> {
        type Item = (&'a ChunkIndex, &'a Chunk);
        fn next(&mut self) -> Option<Self::Item> {
            self.inner.next()
        }
    }


    pub struct IterMut2<'a> {
        inner: std::collections::hash_map::IterMut<'a, ChunkIndex, Chunk>,
    }

    impl<'a> Iterator for IterMut2<'a> {
        type Item = (&'a ChunkIndex, &'a mut Chunk);

        fn next(&mut self) -> Option<Self::Item> {
            self.inner.next()
        }
    }

}
