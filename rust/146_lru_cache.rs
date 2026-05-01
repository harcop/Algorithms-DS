use std::collections::{HashMap, VecDeque};

/// LeetCode #146 - LRU Cache (O(n) eviction; fine for typical constraints in tests)
pub struct LruCache {
    cap: usize,
    map: HashMap<i32, i32>,
    order: VecDeque<i32>,
}

impl LruCache {
    fn new(capacity: i32) -> Self {
        LruCache {
            cap: capacity as usize,
            map: HashMap::new(),
            order: VecDeque::new(),
        }
    }

    fn get(&mut self, key: i32) -> i32 {
        if let Some(&v) = self.map.get(&key) {
            if let Some(i) = self.order.iter().position(|&k| k == key) {
                self.order.remove(i);
            }
            self.order.push_back(key);
            v
        } else {
            -1
        }
    }

    fn put(&mut self, key: i32, value: i32) {
        if self.map.contains_key(&key) {
            self.map.insert(key, value);
            if let Some(i) = self.order.iter().position(|&k| k == key) {
                self.order.remove(i);
            }
            self.order.push_back(key);
            return;
        }
        if self.map.len() >= self.cap {
            if let Some(k) = self.order.pop_front() {
                self.map.remove(&k);
            }
        }
        self.map.insert(key, value);
        self.order.push_back(key);
    }
}

fn main() {
    let mut c = LruCache::new(2);
    c.put(1, 1);
    c.put(2, 2);
    println!("{}", c.get(1));
}

#[cfg(test)]
mod tests {
    use super::LruCache;

    #[test]
    fn example_one() {
        let mut l = LruCache::new(2);
        l.put(1, 1);
        l.put(2, 2);
        assert_eq!(l.get(1), 1);
        l.put(3, 3);
        assert_eq!(l.get(2), -1);
        l.put(4, 4);
        assert_eq!(l.get(1), -1);
        assert_eq!(l.get(3), 3);
        assert_eq!(l.get(4), 4);
    }
}
