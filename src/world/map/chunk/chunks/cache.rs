use std::cell::RefCell;
use std::ops::Deref;

pub static mut CACHE_MISSES: i64 = 0;
pub static mut CACHE_HOT_HITS: i64 = 0;
pub static mut CACHE_COLD_HITS: i64 = 0;

pub fn print_cache_stats() {
    unsafe {
        let total_requests = (CACHE_HOT_HITS + CACHE_COLD_HITS + CACHE_MISSES) as f64;
        if total_requests > 0.0 {
            println!(
                "Cache hits: hot: {}, cold: {}. cache misses: {}, ratio cached: {}, ratio hot: {}",
                CACHE_HOT_HITS,
                CACHE_COLD_HITS,
                CACHE_MISSES,
                (CACHE_HOT_HITS + CACHE_COLD_HITS) as f64 / total_requests,
                CACHE_HOT_HITS as f64 / total_requests
            );
        }
    }
}

pub fn record_cache_miss() {
    unsafe {
        // done in 2 statements because I don't know how to make CLion to show me the static
        // variable while debugging with breakpoints, but the local variable shows up even in
        // release mode
        let current_misses = CACHE_MISSES;
        CACHE_MISSES = current_misses + 1;
    }
}

pub fn record_cache_hot_hit() {
    unsafe {
        let current_hits = CACHE_HOT_HITS;
        CACHE_HOT_HITS = current_hits + 1;
    }
}

pub fn record_cache_cold_hit() {
    unsafe {
        let current_hits = CACHE_COLD_HITS;
        CACHE_COLD_HITS = current_hits + 1;
    }
}

pub struct IndexCache {
    wrapped_cache: RefCell<[usize; 2]>,
}

/// Small cache for indexes.
///
/// Assumes that the 0th index is valid, and only provides 2 cached values.
///
/// For the initial purpose of this class, the first cached value is a cache hit 83% of the times,
/// and when using the second value as well the hit rate goes to 97%.
/// If your access pattern doesn't fit these numbers, you probably need a different cache.
impl IndexCache {
    pub fn new() -> Self {
        Self {
            wrapped_cache: RefCell::new([0, 0]),
        }
    }

    pub fn add_to_cache(&self, i: usize) {
        let mut cache = self.wrapped_cache.take();
        let front = cache[0];
        if i != front {
            cache[1] = front;
            cache[0] = i;
        }
        self.wrapped_cache.replace(cache);
    }

    pub fn get_hot_cached_index(&self) -> usize {
        self.wrapped_cache.borrow().deref()[0]
    }

    pub fn get_cold_cached_index(&self) -> usize {
        self.wrapped_cache.borrow().deref()[1]
    }
}
