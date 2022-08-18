
use std::cell::RefCell;

// pub use hash_impl::{Chunks, IntoIter};
pub use vec_impl::{Chunks, IntoIter};

pub static mut CACHE_MISSES : i64 = 0;
pub static mut CACHE_HOT_HITS : i64 = 0;
pub static mut CACHE_COLD_HITS : i64 = 0;


pub fn print_cache_stats() {
    unsafe {
        let total_requests = (CACHE_HOT_HITS + CACHE_COLD_HITS + CACHE_MISSES) as f64;
        println!("Cache hits: hot: {}, cold: {}. cache misses: {}, ratio cached: {}, ratio hot: {}",
                 CACHE_HOT_HITS,
                 CACHE_COLD_HITS,
                 CACHE_MISSES,
                 (CACHE_HOT_HITS + CACHE_COLD_HITS) as f64 / total_requests,
                 CACHE_HOT_HITS as f64 / total_requests);
    }
}

mod vec_impl {
    use std::cell::RefCell;
    use std::collections::{HashMap, VecDeque};
    use std::ops::{Deref, DerefMut};
    use crate::world::map::chunk::{Chunk, ChunkIndex};
    use super::{CACHE_HOT_HITS, CACHE_COLD_HITS, CACHE_MISSES};

    fn record_cache_miss() {
        unsafe {
            let current_misses = CACHE_MISSES;
            CACHE_MISSES = current_misses + 1;
        }
    }

    fn record_cache_hot_hit() {
        unsafe {
            let current_hits = CACHE_HOT_HITS;
            CACHE_HOT_HITS = current_hits + 1;
        }
    }


    fn record_cache_cold_hit() {
        unsafe {
            let current_hits = CACHE_COLD_HITS;
            CACHE_COLD_HITS = current_hits + 1;
        }
    }

    fn add_to_cache(i: usize, wrapped_cache: &RefCell<VecDeque<usize>>) {
        let mut cache = wrapped_cache.take();
        assert_eq!(cache.len(), 2);
        if !(i.eq(cache.front().unwrap())) {
            cache.push_front(i);
            cache.pop_back();
        }
        wrapped_cache.replace(cache);
    }

    // pub type Chunks = HashMap<ChunkIndex, Chunk>;
    pub type IntoIter = IntoIter2;


    pub struct Chunks {
        inner_vec: Vec<(ChunkIndex, Chunk)>,
        inner_map: HashMap<ChunkIndex, usize>,
        recently_used: RefCell<VecDeque<usize>>,
    }

    type ChunkEntry = (ChunkIndex, Chunk);

    impl Chunks {
        pub fn new() -> Self {
            Chunks {
                inner_vec: Vec::new(),
                inner_map: HashMap::new(),
                recently_used: RefCell::new(VecDeque::from(vec![0, 0])),
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

        pub fn get_mut(&mut self, chunk_index: &ChunkIndex) -> Option<&mut Chunk> {
            for (i, (present_chunk_index, present_chunk)) in self.inner_vec.iter_mut().enumerate() {
                if (*present_chunk_index).eq(chunk_index) {
                    add_to_cache(i, &self.recently_used);
                    return Option::Some(present_chunk);
                }
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
                add_to_cache(*i, &self.recently_used);
                return Option::Some(&self.inner_vec.get(*i).unwrap().1)
            }
            return Option::None
        }

        fn try_hot_cache(&self, chunk_index: &ChunkIndex) -> Option<&Chunk> {
            if let Option::Some(i) = self.get_hot_cached_index() {
                if let Option::Some(entry) = self.inner_vec.get(i) {
                    if entry.0.eq(chunk_index) {
                        record_cache_hot_hit();
                        return Option::Some(&entry.1)
                    }
                }
            }
            Option::None
        }

        fn get_hot_cached_index(&self) -> Option<usize> {
            self.recently_used.borrow().deref().front().copied()
        }

        fn try_cold_cache(&self, chunk_index: &ChunkIndex) -> Option<&Chunk> {
            let i_option = self.get_cold_cached_index();
            if let Option::Some(i) = i_option {
                if let Option::Some(entry) = self.inner_vec.get(i) {
                    if entry.0.eq(chunk_index) {
                        add_to_cache(i, &self.recently_used);
                        record_cache_cold_hit();
                        return Option::Some(&entry.1)
                    }
                }
            }
            Option::None
        }

        fn get_cold_cached_index(&self) -> Option<usize> {
            self.recently_used.borrow().deref().back().copied()
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
                recently_used: RefCell::new(VecDeque::from(vec![0, 0])),
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
    pub type IntoIter = IntoIter2;

    pub struct Chunks {
        inner_map: HashMap<ChunkIndex, Chunk>,
    }

    type ChunkEntry = (ChunkIndex, Chunk);

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

mod list {
pub struct List<T> {
    head: Link<T>,
}

type Link<T> = Option<Box<Node<T>>>;

struct Node<T> {
    elem: T,
    next: Link<T>,
}

impl<T> List<T> {
    pub fn new() -> Self {
        List { head: None }
    }

    pub fn push(&mut self, elem: T) {
        let new_node = Box::new(Node {
            elem: elem,
            next: self.head.take(),
        });

        self.head = Some(new_node);
    }

    pub fn pop(&mut self) -> Option<T> {
        self.head.take().map(|node| {
            self.head = node.next;
            node.elem
        })
    }

    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|node| {
            &node.elem
        })
    }

    pub fn peek_mut(&mut self) -> Option<&mut T> {
        self.head.as_mut().map(|node| {
            &mut node.elem
        })
    }

    pub fn into_iter(self) -> IntoIter<T> {
        IntoIter(self)
    }

    pub fn iter(&self) -> Iter<'_, T> {
        Iter { next: self.head.as_deref() }
    }

    pub fn iter_mut(&mut self) -> IterMut<'_, T> {
        IterMut { next: self.head.as_deref_mut() }
    }
}

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        let mut cur_link = self.head.take();
        while let Some(mut boxed_node) = cur_link {
            cur_link = boxed_node.next.take();
        }
    }
}

pub struct IntoIter<T>(List<T>);

impl<T> Iterator for IntoIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop()
    }
}

pub struct Iter<'a, T> {
    next: Option<&'a Node<T>>,
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|node| {
            self.next = node.next.as_deref();
            &node.elem
        })
    }
}

pub struct IterMut<'a, T> {
    next: Option<&'a mut Node<T>>,
}

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.take().map(|node| {
            self.next = node.next.as_deref_mut();
            &mut node.elem
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn basics() {
        let mut list = List::new();

        // Check empty list behaves right
        assert_eq!(list.pop(), None);

        // Populate list
        list.push(1);
        list.push(2);
        list.push(3);

        // Check normal removal
        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.pop(), Some(2));

        // Push some more just to make sure nothing's corrupted
        list.push(4);
        list.push(5);

        // Check normal removal
        assert_eq!(list.pop(), Some(5));
        assert_eq!(list.pop(), Some(4));

        // Check exhaustion
        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.pop(), None);
    }

    #[test]
    fn peek() {
        let mut list = List::new();
        assert_eq!(list.peek(), None);
        assert_eq!(list.peek_mut(), None);
        list.push(1); list.push(2); list.push(3);

        assert_eq!(list.peek(), Some(&3));
        assert_eq!(list.peek_mut(), Some(&mut 3));

        list.peek_mut().map(|value| {
            *value = 42
        });

        assert_eq!(list.peek(), Some(&42));
        assert_eq!(list.pop(), Some(42));
    }

    #[test]
    fn into_iter() {
        let mut list = List::new();
        list.push(1); list.push(2); list.push(3);

        let mut iter = list.into_iter();
        assert_eq!(iter.next(), Some(3));
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn iter() {
        let mut list = List::new();
        list.push(1); list.push(2); list.push(3);

        let mut iter = list.iter();
        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&1));
    }

    #[test]
    fn iter_mut() {
        let mut list = List::new();
        list.push(1); list.push(2); list.push(3);

        let mut iter = list.iter_mut();
        assert_eq!(iter.next(), Some(&mut 3));
        assert_eq!(iter.next(), Some(&mut 2));
        assert_eq!(iter.next(), Some(&mut 1));
    }
}


}
