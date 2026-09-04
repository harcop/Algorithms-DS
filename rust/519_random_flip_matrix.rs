/// LeetCode #519 - Random Flip Matrix
use std::collections::HashMap;

struct Solution {
    m: i32,
    n: i32,
    remaining: i32,
    map: HashMap<i32, i32>,
    rng: u64,
}

impl Solution {
    fn new(m: i32, n: i32) -> Self {
        Solution {
            m,
            n,
            remaining: m * n,
            map: HashMap::new(),
            rng: 0x9e3779b97f4a7c15,
        }
    }

    fn next_rand(&mut self) -> u64 {
        self.rng = self.rng.wrapping_mul(6364136223846793005).wrapping_add(1);
        self.rng
    }

    fn flip(&mut self) -> Vec<i32> {
        let r = (self.next_rand() % self.remaining as u64) as i32;
        self.remaining -= 1;
        let x = *self.map.get(&r).unwrap_or(&r);
        let last = *self.map.get(&self.remaining).unwrap_or(&self.remaining);
        self.map.insert(r, last);
        vec![x / self.n, x % self.n]
    }

    fn reset(&mut self) {
        self.remaining = self.m * self.n;
        self.map.clear();
    }
}

fn main() {
    let mut s = Solution::new(3, 1);
    println!("{:?}", s.flip());
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use std::collections::HashSet;

    #[test]
    fn example_one() {
        let mut s = Solution::new(3, 1);
        let mut seen = HashSet::new();
        for _ in 0..3 {
            let p = s.flip();
            assert_eq!(p.len(), 2);
            assert!(p[0] >= 0 && p[0] < 3);
            assert_eq!(p[1], 0);
            assert!(seen.insert((p[0], p[1])));
        }
        s.reset();
        let p = s.flip();
        assert!(p[0] >= 0 && p[0] < 3);
        assert_eq!(p[1], 0);
    }
}
