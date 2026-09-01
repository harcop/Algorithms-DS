/// LeetCode #3520 - Minimum Threshold for Inversion Pairs Count
struct Fenwick {
    bit: Vec<i32>,
}

impl Fenwick {
    fn new(n: usize) -> Self {
        Self { bit: vec![0; n + 1] }
    }

    fn add(&mut self, mut i: usize, v: i32) {
        while i < self.bit.len() {
            self.bit[i] += v;
            i += i & i.wrapping_neg();
        }
    }

    fn prefix(&self, mut i: usize) -> i32 {
        let mut s = 0;
        while i > 0 {
            s += self.bit[i];
            i -= i & i.wrapping_neg();
        }
        s
    }

    fn range(&self, l: usize, r: usize) -> i32 {
        if l > r {
            0
        } else {
            self.prefix(r) - self.prefix(l - 1)
        }
    }
}

fn count_pairs(nums: &[i32], x: i32, vals: &[i32]) -> i64 {
    let m = vals.len();
    let mut fen = Fenwick::new(m);
    let rank = |v: i32| vals.partition_point(|&y| y < v);
    let mut ans = 0i64;
    for &v in nums {
        let lo = rank(v + 1) + 1;
        let hi = rank(v + x + 1);
        if lo <= hi {
            ans += fen.range(lo, hi) as i64;
        }
        fen.add(rank(v) + 1, 1);
    }
    ans
}

fn min_threshold(nums: Vec<i32>, k: i32) -> i32 {
    let mut vals = nums.clone();
    vals.sort_unstable();
    vals.dedup();
    let k = k as i64;
    let max_x = *nums.iter().max().unwrap();
    let mut lo = 0;
    let mut hi = max_x;
    let mut ans = -1;
    while lo <= hi {
        let mid = lo + (hi - lo) / 2;
        if count_pairs(&nums, mid, &vals) >= k {
            ans = mid;
            hi = mid - 1;
        } else {
            lo = mid + 1;
        }
    }
    ans
}

fn main() {
    println!("{}", min_threshold(vec![1, 2, 3, 4, 3, 2, 1], 7));
}

#[cfg(test)]
mod tests {
    use super::min_threshold;

    #[test]
    fn example1() {
        assert_eq!(min_threshold(vec![1, 2, 3, 4, 3, 2, 1], 7), 2);
    }

    #[test]
    fn example2() {
        assert_eq!(min_threshold(vec![10, 9, 9, 9, 1], 4), 8);
    }
}
