/// LeetCode #2622 - Cache With Time Limit (JS problem; Rust Instant analogue)
use std::collections::HashMap;
use std::time::{Duration, Instant};

struct TimeLimitedCache {
    cache: HashMap<i32, (i32, Instant)>,
}

impl TimeLimitedCache {
    fn new() -> Self {
        TimeLimitedCache {
            cache: HashMap::new(),
        }
    }

    fn set(&mut self, key: i32, value: i32, duration: u64) -> bool {
        let existed = self.cache.contains_key(&key) && !self.is_expired(key);
        self.cache
            .insert(key, (value, Instant::now() + Duration::from_millis(duration)));
        existed
    }

    fn get(&mut self, key: i32) -> i32 {
        if self.is_expired(key) {
            -1
        } else {
            self.cache.get(&key).map(|(v, _)| *v).unwrap_or(-1)
        }
    }

    fn count(&mut self) -> i32 {
        let keys: Vec<i32> = self.cache.keys().copied().collect();
        keys.into_iter().filter(|&k| !self.is_expired(k)).count() as i32
    }

    fn is_expired(&mut self, key: i32) -> bool {
        match self.cache.get(&key) {
            Some((_, exp)) if *exp > Instant::now() => false,
            Some(_) => {
                self.cache.remove(&key);
                true
            }
            None => true,
        }
    }
}

fn main() {
    let mut obj = TimeLimitedCache::new();
    println!("{}", obj.set(1, 42, 1000));
    println!("{}", obj.get(1));
    println!("{}", obj.count());
}

#[cfg(test)]
mod tests {
    use super::TimeLimitedCache;
    use std::thread;
    use std::time::Duration;

    #[test]
    fn example_behaviour() {
        let mut cache = TimeLimitedCache::new();
        assert!(!cache.set(1, 42, 50));
        assert_eq!(cache.get(1), 42);
        assert_eq!(cache.count(), 1);
        thread::sleep(Duration::from_millis(60));
        assert_eq!(cache.get(1), -1);
        assert_eq!(cache.count(), 0);
    }

    #[test]
    fn overwrite_unexpired() {
        let mut cache = TimeLimitedCache::new();
        assert!(!cache.set(1, 42, 100));
        assert!(cache.set(1, 50, 100));
        assert_eq!(cache.get(1), 50);
    }
}
