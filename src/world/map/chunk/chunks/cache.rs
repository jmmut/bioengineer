use std::cell::RefCell;

pub static mut CACHE_MISSES : i64 = 0;
pub static mut CACHE_HOT_HITS : i64 = 0;
pub static mut CACHE_COLD_HITS : i64 = 0;

pub fn print_cache_stats() {
    unsafe {
        let total_requests = (CACHE_HOT_HITS + CACHE_COLD_HITS + CACHE_MISSES) as f64;
        if total_requests > 0.0 {
            println!("Cache hits: hot: {}, cold: {}. cache misses: {}, ratio cached: {}, ratio hot: {}",
                     CACHE_HOT_HITS,
                     CACHE_COLD_HITS,
                     CACHE_MISSES,
                     (CACHE_HOT_HITS + CACHE_COLD_HITS) as f64 / total_requests,
                     CACHE_HOT_HITS as f64 / total_requests);
        }
    }
}


    pub fn record_cache_miss() {
        unsafe {
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

    pub fn add_to_cache(i: usize, wrapped_cache: &RefCell<[usize; 2]>) {
        let mut cache = wrapped_cache.take();
        let front = cache[0];
        if i != front {
            cache[1] = front;
            cache[0] = i;
        }
        wrapped_cache.replace(cache);
    }

