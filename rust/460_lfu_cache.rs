/// LeetCode #460 - LFU Cache
use std::collections::{HashMap, VecDeque};

struct LfuCache {
    cap: usize,
    /// current minimum frequency among keys in the cache (used for eviction)
    min_f: usize,
    key_val: HashMap<i32, i32>,
    key_freq: HashMap<i32, usize>,
    /// freq -> queue of keys used in LRU order within the same frequency bucket
    freq_bucket: HashMap<usize, VecDeque<i32>>,
}

impl LfuCache {
    fn new(capacity: i32) -> Self {
        LfuCache {
            cap: capacity as usize,
            min_f: 0,
            key_val: HashMap::new(),
            key_freq: HashMap::new(),
            freq_bucket: HashMap::new(),
        }
    }

    fn remove_key_from_bucket(&mut self, key: i32, freq: usize) {
        let dq = self.freq_bucket.get_mut(&freq).unwrap();
        if let Some(idx) = dq.iter().position(|&k| k == key) {
            dq.remove(idx).unwrap();
        }
        if dq.is_empty() {
            self.freq_bucket.remove(&freq);
            if freq == self.min_f {
                self.min_f += 1;
            }
        }
    }

    fn touch(&mut self, key: i32) {
        let f = self.key_freq[&key];
        self.remove_key_from_bucket(key, f);
        let nf = f + 1;
        self.key_freq.insert(key, nf);
        self.freq_bucket.entry(nf).or_default().push_back(key);
    }

    fn get(&mut self, key: i32) -> i32 {
        if !self.key_val.contains_key(&key) {
            return -1;
        }
        self.touch(key);
        *self.key_val.get(&key).unwrap()
    }

    fn put(&mut self, key: i32, value: i32) {
        if self.cap == 0 {
            return;
        }

        if let Some(v) = self.key_val.get_mut(&key) {
            *v = value;
            self.touch(key);
            return;
        }

        if self.key_val.len() == self.cap {
            let victim = {
                let dq = self.freq_bucket.get_mut(&self.min_f).unwrap();
                dq.pop_front().unwrap()
            };
            if self.freq_bucket[&self.min_f].is_empty() {
                self.freq_bucket.remove(&self.min_f);
            }
            self.key_val.remove(&victim);
            self.key_freq.remove(&victim);
        }

        self.key_val.insert(key, value);
        self.key_freq.insert(key, 1);
        self.freq_bucket.entry(1).or_default().push_back(key);
        self.min_f = 1;
    }
}

fn main() {
    let mut c = LfuCache::new(2);
    c.put(1, 1);
    println!("{}", c.get(1));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lc() {
        let mut c = LfuCache::new(2);
        c.put(1, 1);
        c.put(2, 2);
        assert_eq!(c.get(1), 1);
        c.put(3, 3);
        assert_eq!(c.get(2), -1);
        assert_eq!(c.get(3), 3);
        c.put(4, 4);
        assert_eq!(c.get(1), -1);
        assert_eq!(c.get(3), 3);
        assert_eq!(c.get(4), 4);
    }
}
