use crate::now;

/// needs a function `now` that returns a timestamp in seconds.
/// `now` should be extracted as a generic parameter if taken out of this project.
pub struct ScopedProfiler {
    pub enabled: bool,
    pub start_ts: f64,
    pub name: Option<String>,
}

impl ScopedProfiler {
    #[allow(dead_code)]
    pub fn new(enabled: bool) -> Self {
        Self::new_with_maybe_name(enabled, Option::None)
    }
    pub fn new_named(enabled: bool, name: &str) -> Self {
        Self::new_with_maybe_name(enabled, Option::Some(String::from(name)))
    }
    fn new_with_maybe_name(enabled: bool, name: Option<String>) -> Self {
        let start_ts = if enabled {
            now()
        } else {
            0.0
        };
        Self {
            enabled,
            start_ts,
            name,
        }
    }
}

impl Drop for ScopedProfiler {
    fn drop(&mut self) {
        if self.enabled {
            let diff = now() - self.start_ts;
            let formatted_name = match &self.name {
                None => {String::new()}
                Some(name) => {
                    format!(" on: {}", name)
                }
            };
            println!("Spent: {:.3} ms{}", diff * 1000.0, formatted_name);
        }
    }
}
