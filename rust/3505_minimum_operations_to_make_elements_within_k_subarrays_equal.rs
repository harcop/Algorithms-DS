/// LeetCode #3505 - Minimum Operations to Make Elements Within K Subarrays Equal
use std::collections::BTreeMap;

struct MultiSet {
    map: BTreeMap<i32, i32>,
    size: usize,
    sum: i64,
}

impl MultiSet {
    fn new() -> Self {
        Self {
            map: BTreeMap::new(),
            size: 0,
            sum: 0,
        }
    }

    fn insert(&mut self, x: i32) {
        *self.map.entry(x).or_insert(0) += 1;
        self.size += 1;
        self.sum += x as i64;
    }

    fn remove_one(&mut self, x: i32) -> bool {
        let Some(c) = self.map.get_mut(&x) else {
            return false;
        };
        *c -= 1;
        self.size -= 1;
        self.sum -= x as i64;
        if *c == 0 {
            self.map.remove(&x);
        }
        true
    }

    fn min_key(&self) -> i32 {
        *self.map.keys().next().unwrap()
    }

    fn max_key(&self) -> i32 {
        *self.map.keys().next_back().unwrap()
    }

    fn is_empty(&self) -> bool {
        self.size == 0
    }
}

fn get_min_ops(nums: &[i32], x: usize) -> Vec<i64> {
    let mut lower = MultiSet::new();
    let mut upper = MultiSet::new();
    let mut min_ops = Vec::new();
    for i in 0..nums.len() {
        if lower.is_empty() || nums[i] <= lower.max_key() {
            lower.insert(nums[i]);
        } else {
            upper.insert(nums[i]);
        }
        if i >= x {
            let out = nums[i - x];
            if !lower.remove_one(out) {
                upper.remove_one(out);
            }
        }
        if lower.size < upper.size {
            let val = upper.min_key();
            upper.remove_one(val);
            lower.insert(val);
        } else if lower.size > upper.size + 1 {
            let val = lower.max_key();
            lower.remove_one(val);
            upper.insert(val);
        }
        if i + 1 >= x {
            let median = lower.max_key() as i64;
            let ops = (median * lower.size as i64 - lower.sum)
                + (upper.sum - median * upper.size as i64);
            min_ops.push(ops);
        }
    }
    min_ops
}

fn min_operations(nums: Vec<i32>, x: i32, k: i32) -> i64 {
    let n = nums.len();
    let x = x as usize;
    let k = k as usize;
    let min_ops = get_min_ops(&nums, x);
    const INF: i64 = i64::MAX / 4;
    let mut cost = vec![INF; n + 1];
    for (i, &c) in min_ops.iter().enumerate() {
        cost[i + x] = c;
    }
    let mut dp = vec![0i64; n + 1];
    for _ in 0..k {
        let mut new_dp = vec![INF; n + 1];
        for j in x..=n {
            new_dp[j] = new_dp[j - 1].min(dp[j - x] + cost[j]);
        }
        dp = new_dp;
    }
    dp[n]
}

fn main() {
    println!(
        "{}",
        min_operations(vec![5, -2, 1, 3, 7, 3, 6, 4, -1], 3, 2)
    );
}

#[cfg(test)]
mod tests {
    use super::min_operations;

    #[test]
    fn example1() {
        assert_eq!(
            min_operations(vec![5, -2, 1, 3, 7, 3, 6, 4, -1], 3, 2),
            8
        );
    }

    #[test]
    fn example2() {
        assert_eq!(min_operations(vec![9, -2, -2, -2, 1, 5], 2, 2), 3);
    }
}
