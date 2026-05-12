/// LeetCode #710 - Random Pick with Blacklist
use std::collections::{HashMap, HashSet};

struct Solution {
    bound: i32,
    map: HashMap<i32, i32>,
    seed: u64,
}

impl Solution {
    fn new(n: i32, blacklist: Vec<i32>) -> Self {
        let bound = n - blacklist.len() as i32;
        let blackset: HashSet<i32> = blacklist.iter().copied().collect();
        let mut next = bound;
        let mut map = HashMap::new();
        for &x in &blacklist {
            if x < bound {
                while blackset.contains(&next) {
                    next += 1;
                }
                map.insert(x, next);
                next += 1;
            }
        }
        Self {
            bound,
            map,
            seed: 12345,
        }
    }

    fn pick(&mut self) -> i32 {
        self.seed = self.seed.wrapping_mul(1103515245).wrapping_add(12345);
        let r = (self.seed % self.bound as u64) as i32;
        *self.map.get(&r).unwrap_or(&r)
    }
}

fn main() {
    let mut s = Solution::new(7, vec![2, 3, 5]);
    println!("{}", s.pick());
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn never_returns_blacklisted() {
        let blacklist = vec![2, 3, 5];
        let mut s = Solution::new(7, blacklist.clone());
        let blackset: std::collections::HashSet<i32> = blacklist.into_iter().collect();
        for _ in 0..200 {
            let v = s.pick();
            assert!(!blackset.contains(&v));
            assert!(v >= 0 && v < 7);
        }
    }

    #[test]
    fn no_blacklist() {
        let mut s = Solution::new(5, vec![]);
        for _ in 0..50 {
            let v = s.pick();
            assert!(v >= 0 && v < 5);
        }
    }
}
