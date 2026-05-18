/// LeetCode #981 - Time Based Key-Value Store
use std::collections::HashMap;

struct TimeMap {
    store: HashMap<String, Vec<(i32, String)>>,
}

impl TimeMap {
    fn new() -> Self {
        TimeMap {
            store: HashMap::new(),
        }
    }

    fn set(&mut self, key: String, value: String, timestamp: i32) {
        self.store.entry(key).or_default().push((timestamp, value));
    }

    fn get(&self, key: String, timestamp: i32) -> String {
        let Some(entries) = self.store.get(&key) else {
            return String::new();
        };
        let mut lo = 0usize;
        let mut hi = entries.len();
        while lo < hi {
            let mid = lo + (hi - lo) / 2;
            if entries[mid].0 <= timestamp {
                lo = mid + 1;
            } else {
                hi = mid;
            }
        }
        if lo == 0 {
            String::new()
        } else {
            entries[lo - 1].1.clone()
        }
    }
}

fn main() {
    let mut map = TimeMap::new();
    map.set("foo".into(), "bar".into(), 1);
    println!("{}", map.get("foo".into(), 1));
}

#[cfg(test)]
mod tests {
    use super::TimeMap;

    #[test]
    fn example_one() {
        let mut map = TimeMap::new();
        map.set("foo".into(), "bar".into(), 1);
        assert_eq!(map.get("foo".into(), 1), "bar");
        assert_eq!(map.get("foo".into(), 3), "bar");
        map.set("foo".into(), "bar2".into(), 4);
        assert_eq!(map.get("foo".into(), 4), "bar2");
        assert_eq!(map.get("foo".into(), 5), "bar2");
    }
}
