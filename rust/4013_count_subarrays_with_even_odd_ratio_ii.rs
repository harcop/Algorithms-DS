/// LeetCode #4013 - Count Subarrays With Even Odd Ratio II
struct BinaryIndexedTree {
    n: usize,
    c: Vec<i32>,
}

impl BinaryIndexedTree {
    fn new(n: usize) -> Self {
        Self {
            n,
            c: vec![0; n + 1],
        }
    }

    fn update(&mut self, mut x: usize, delta: i32) {
        while x <= self.n {
            self.c[x] += delta;
            x += x & x.wrapping_neg();
        }
    }

    fn query(&self, mut x: usize) -> i32 {
        let mut s = 0;
        while x > 0 {
            s += self.c[x];
            x &= x - 1;
        }
        s
    }
}

fn count_ratio_subarrays(nums: Vec<i32>, a: i32, b: i32) -> i64 {
    let n = nums.len();
    let mut s = vec![0i64; n + 1];
    for i in 0..n {
        s[i + 1] = s[i] + if nums[i] % 2 == 1 { a as i64 } else { -(b as i64) };
    }
    let mut sorted = s.clone();
    sorted.sort_unstable();
    sorted.dedup();
    let mut bit = BinaryIndexedTree::new(sorted.len());
    let mut ans = 0i64;
    for v in s {
        let x = sorted.binary_search(&v).unwrap() + 1;
        ans += bit.query(x) as i64;
        bit.update(x, 1);
    }
    ans
}

fn main() {
    println!("{}", count_ratio_subarrays(vec![1, 2, 1, 2], 3, 2));
}

#[cfg(test)]
mod tests {
    use super::count_ratio_subarrays;

    #[test]
    fn example1() {
        assert_eq!(count_ratio_subarrays(vec![1, 2, 1, 2], 3, 2), 7);
    }

    #[test]
    fn example2() {
        assert_eq!(count_ratio_subarrays(vec![2, 2, 1], 2, 1), 3);
    }

    #[test]
    fn example3() {
        assert_eq!(count_ratio_subarrays(vec![2, 2, 2], 1, 1), 0);
    }
}
