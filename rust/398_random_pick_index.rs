/// LeetCode #398 - Random Pick Index (uniform among indices matching `pick` target)
use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};

struct Solution {
    m: HashMap<i32, Vec<usize>>,
}

impl Solution {
    fn new(nums: Vec<i32>) -> Self {
        let mut m = HashMap::new();
        for (i, x) in nums.into_iter().enumerate() {
            m.entry(x).or_insert_with(Vec::new).push(i);
        }
        Solution { m }
    }

    fn pick(&self, target: i32) -> i32 {
        let v = &self.m[&target];
        let r = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .subsec_nanos() as usize
            % v.len();
        v[r] as i32
    }
}

fn main() {
    let s = Solution::new(vec![1, 2, 3, 3, 3]);
    println!("{}", s.pick(3));
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn picks() {
        let s = Solution::new(vec![1, 2, 3, 3, 3]);
        let i = s.pick(3);
        assert!([2, 3, 4].contains(&(i as usize)));
    }
}
