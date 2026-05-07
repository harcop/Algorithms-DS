/// LeetCode #432 - All O`one` Data Structure (freq buckets via `HashMap` + coarse min/max scans)
use std::collections::{BTreeMap, HashMap};

struct AllOne {
    key_to_freq: HashMap<String, i32>,
    freq_to_keys: HashMap<i32, BTreeMap<String, ()>>, // alphabetical tie-break not required here
}

impl AllOne {
    fn new() -> Self {
        AllOne {
            key_to_freq: HashMap::new(),
            freq_to_keys: HashMap::new(),
        }
    }

    fn inc(&mut self, key: String) {
        let f = self.key_to_freq.entry(key.clone()).or_insert(0);
        let old = *f;
        *f += 1;
        let new = *f;
        self.freq_to_keys.entry(old).or_default().remove(&key);
        if self.freq_to_keys.get(&old).map(|m| m.is_empty()).unwrap_or(false) {
            self.freq_to_keys.remove(&old);
        }
        self.freq_to_keys.entry(new).or_default().insert(key, ());
    }

    fn dec(&mut self, key: String) {
        let f = self.key_to_freq.get_mut(&key).unwrap();
        let old = *f;
        *f -= 1;
        let new = *f;
        self.freq_to_keys.entry(old).or_default().remove(&key);
        if self.freq_to_keys.get(&old).map(|m| m.is_empty()).unwrap_or(false) {
            self.freq_to_keys.remove(&old);
        }
        if new == 0 {
            self.key_to_freq.remove(&key);
        } else {
            self.freq_to_keys.entry(new).or_default().insert(key, ());
        }
    }

    fn get_max_key(&self) -> String {
        self.freq_to_keys.keys().copied().max().and_then(|f| self.freq_to_keys[&f].first_key_value().map(|(k, _)| k.clone())).unwrap_or_default()
    }

    fn get_min_key(&self) -> String {
        self.freq_to_keys.keys().copied().min().and_then(|f| self.freq_to_keys[&f].first_key_value().map(|(k, _)| k.clone())).unwrap_or_default()
    }
}

fn main() {
    let mut o = AllOne::new();
    o.inc("a".into());
    println!("{}", o.get_max_key());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lc_ops() {
        let mut ao = AllOne::new();
        ao.inc("hello".into());
        ao.inc("hello".into());
        assert_eq!(ao.get_max_key(), "hello");
        assert_eq!(ao.get_min_key(), "hello");
        ao.inc("leet".into());
        assert_eq!(ao.get_max_key(), "hello");
        ao.dec("hello".into());
        ao.dec("hello".into());
        assert_eq!(ao.get_max_key(), "leet");
    }
}
