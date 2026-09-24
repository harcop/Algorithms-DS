/// LeetCode #3915 - Maximum Sum of Alternating Subsequence With Distance at Least K
struct MaxBit {
    c: Vec<i64>,
}

impl MaxBit {
    fn new(n: usize) -> Self {
        Self { c: vec![0; n + 1] }
    }

    fn update(&mut self, mut x: usize, val: i64) {
        while x < self.c.len() {
            self.c[x] = self.c[x].max(val);
            x += x & x.wrapping_neg();
        }
    }

    fn query(&self, mut x: usize) -> i64 {
        let mut ans = 0i64;
        while x > 0 {
            ans = ans.max(self.c[x]);
            x -= x & x.wrapping_neg();
        }
        ans
    }
}

fn max_alternating_sum(nums: Vec<i32>, k: i32) -> i64 {
    let mut sorted = nums.clone();
    sorted.sort_unstable();
    sorted.dedup();
    let m = sorted.len();
    let rank = |x: i32| -> usize { sorted.binary_search(&x).unwrap() + 1 };
    let mut bit0 = MaxBit::new(m);
    let mut bit1 = MaxBit::new(m);
    let n = nums.len();
    let k = k as usize;
    let mut f0 = vec![0i64; n];
    let mut f1 = vec![0i64; n];
    let mut ans = 0i64;
    for i in 0..n {
        if i >= k {
            let r = rank(nums[i - k]);
            bit0.update(r, f0[i - k]);
            bit1.update(m + 1 - r, f1[i - k]);
        }
        let x = nums[i] as i64;
        let r = rank(nums[i]);
        f0[i] = x + bit1.query(m - r);
        f1[i] = x + bit0.query(r - 1);
        ans = ans.max(f0[i]).max(f1[i]);
    }
    ans
}

fn main() {
    println!("{}", max_alternating_sum(vec![5, 4, 2], 2));
}

#[cfg(test)]
mod tests {
    use super::max_alternating_sum;

    #[test]
    fn example1() {
        assert_eq!(max_alternating_sum(vec![5, 4, 2], 2), 7);
    }

    #[test]
    fn example2() {
        assert_eq!(max_alternating_sum(vec![3, 5, 4, 2, 4], 1), 14);
    }

    #[test]
    fn example3() {
        assert_eq!(max_alternating_sum(vec![5], 1), 5);
    }
}
