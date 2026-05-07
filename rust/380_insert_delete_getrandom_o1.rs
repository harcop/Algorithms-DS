/// LeetCode #380 - Insert Delete GetRandom O(1)
use std::collections::HashMap;

struct RandomizedSet {
    idx: HashMap<i32, usize>,
    vals: Vec<i32>,
}

impl RandomizedSet {
    fn new() -> Self {
        RandomizedSet {
            idx: HashMap::new(),
            vals: vec![],
        }
    }

    fn insert(&mut self, val: i32) -> bool {
        if self.idx.contains_key(&val) {
            return false;
        }
        let i = self.vals.len();
        self.idx.insert(val, i);
        self.vals.push(val);
        true
    }

    fn remove(&mut self, val: i32) -> bool {
        if let Some(&i) = self.idx.get(&val) {
            let last = *self.vals.last().unwrap();
            self.vals.swap_remove(i);
            if i < self.vals.len() {
                self.idx.insert(last, i);
            }
            self.idx.remove(&val);
            true
        } else {
            false
        }
    }

    fn get_random(&self) -> i32 {
        let i = rand_usize_mod(self.vals.len());
        self.vals[i]
    }
}

fn rand_usize_mod(m: usize) -> usize {
    use std::time::{SystemTime, UNIX_EPOCH};
    let nanos = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().subsec_nanos() as usize;
    nanos % m
}

fn main() {
    let mut s = RandomizedSet::new();
    println!("{}", s.insert(1));
}

#[cfg(test)]
mod tests {
    use super::RandomizedSet;

    #[test]
    fn smoke() {
        let mut r = RandomizedSet::new();
        assert!(r.insert(2));
        assert!(!r.insert(2));
        assert!(r.remove(2));
        assert!(r.insert(3));
        let _ = r.get_random();
    }
}
