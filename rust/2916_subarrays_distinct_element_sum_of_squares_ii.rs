/// LeetCode #2916 - Subarrays Distinct Element Sum of Squares II
struct LazySegmentTree {
    n: usize,
    lazy: Vec<i64>,
    sums: Vec<i64>,
    squared_sums: Vec<i64>,
}

impl LazySegmentTree {
    const MOD: i64 = 1_000_000_007;

    fn new(n: usize) -> Self {
        Self {
            n,
            lazy: vec![0; 4 * n],
            sums: vec![0; 4 * n],
            squared_sums: vec![0; 4 * n],
        }
    }

    fn propagate(&mut self, i: usize, l: usize, r: usize) {
        if self.lazy[i] == 0 {
            return;
        }
        let gap = (r - l + 1) as i64;
        let add = self.lazy[i];
        self.squared_sums[i] =
            (self.squared_sums[i] + 2 * add * self.sums[i] + add * add % Self::MOD * gap) % Self::MOD;
        self.sums[i] = (self.sums[i] + add * gap) % Self::MOD;
        if l < r {
            self.lazy[i * 2 + 1] += add;
            self.lazy[i * 2 + 2] += add;
        }
        self.lazy[i] = 0;
    }

    fn update_range(&mut self, l: usize, r: usize) {
        self.update_range_rec(0, 0, self.n - 1, l, r);
    }

    fn update_range_rec(&mut self, i: usize, start: usize, end: usize, l: usize, r: usize) {
        self.propagate(i, start, end);
        if end < l || start > r {
            return;
        }
        if start >= l && end <= r {
            self.lazy[i] = 1;
            self.propagate(i, start, end);
            return;
        }
        let mid = (start + end) / 2;
        self.update_range_rec(i * 2 + 1, start, mid, l, r);
        self.update_range_rec(i * 2 + 2, mid + 1, end, l, r);
        self.sums[i] = (self.sums[i * 2 + 1] + self.sums[i * 2 + 2]) % Self::MOD;
        self.squared_sums[i] =
            (self.squared_sums[i * 2 + 1] + self.squared_sums[i * 2 + 2]) % Self::MOD;
    }

    fn tree_squared_sums(&self) -> i64 {
        self.squared_sums[0]
    }
}

fn sum_counts(nums: Vec<i32>) -> i32 {
    use std::collections::HashMap;

    const MOD: i64 = 1_000_000_007;
    let n = nums.len();
    let mut ans = 0i64;
    let mut last_seen = HashMap::new();
    let mut tree = LazySegmentTree::new(n);

    for r in 0..n {
        let l = last_seen.get(&nums[r]).map(|&p| p + 1).unwrap_or(0);
        tree.update_range(l, r);
        last_seen.insert(nums[r], r);
        ans = (ans + tree.tree_squared_sums()) % MOD;
    }
    ans as i32
}

fn main() {
    println!("{}", sum_counts(vec![1, 2, 1]));
}

#[cfg(test)]
mod tests {
    use super::sum_counts;

    #[test]
    fn example_one() {
        assert_eq!(sum_counts(vec![1, 2, 1]), 15);
    }

    #[test]
    fn example_two() {
        assert_eq!(sum_counts(vec![2, 2]), 3);
    }
}
