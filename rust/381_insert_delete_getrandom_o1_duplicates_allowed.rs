/// LeetCode #381 - Randomized Collection (multiset with index map)
use std::collections::{HashMap, VecDeque};

struct RandomizedCollection {
    vals: Vec<i32>,
    pos: HashMap<i32, VecDeque<usize>>,
}

impl RandomizedCollection {
    fn new() -> Self {
        RandomizedCollection {
            vals: Vec::new(),
            pos: HashMap::new(),
        }
    }

    fn insert(&mut self, val: i32) -> bool {
        let i = self.vals.len();
        self.vals.push(val);
        let first = {
            let q = self.pos.entry(val).or_default();
            let nf = q.is_empty();
            q.push_back(i);
            nf
        };
        first
    }

    fn remove(&mut self, val: i32) -> bool {
        let q = match self.pos.get_mut(&val) {
            Some(q) if !q.is_empty() => q,
            _ => return false,
        };
        let rm = q.pop_front().unwrap();
        if q.is_empty() {
            self.pos.remove(&val);
        }
        let last = self.vals.len() - 1;
        if rm != last {
            let moved = self.vals[last];
            self.vals[rm] = moved;
            if let Some(mq) = self.pos.get_mut(&moved) {
                for id in mq.iter_mut() {
                    if *id == last {
                        *id = rm;
                        break;
                    }
                }
            }
        }
        self.vals.pop();
        true
    }

    fn get_random(&self) -> i32 {
        let len = self.vals.len().max(1);
        let i = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .subsec_nanos() as usize
            % len;
        self.vals[i]
    }
}

fn main() {
    let mut c = RandomizedCollection::new();
    println!("{}", c.insert(4));
}

#[cfg(test)]
mod tests {
    use super::RandomizedCollection;

    #[test]
    fn multiset() {
        let mut r = RandomizedCollection::new();
        assert!(r.insert(4));
        assert!(!r.insert(4));
        assert!(!r.remove(3));
        assert!(r.remove(4));
        let _ = r.get_random();
    }
}
