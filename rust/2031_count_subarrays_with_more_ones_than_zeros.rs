/// LeetCode #2031 - Count Subarrays With More Ones Than Zeros
const MOD: i64 = 1_000_000_007;

struct Fenwick {
    n: usize,
    c: Vec<i64>,
}

impl Fenwick {
    fn new(n: usize) -> Self {
        Self {
            n,
            c: vec![0; n + 1],
        }
    }

    fn update(&mut self, mut x: usize, v: i64) {
        while x <= self.n {
            self.c[x] += v;
            x += x & x.wrapping_neg();
        }
    }

    fn query(&self, mut x: usize) -> i64 {
        let mut s = 0i64;
        while x > 0 {
            s += self.c[x];
            x -= x & x.wrapping_neg();
        }
        s
    }
}

fn subarrays_with_more_zeros_than_ones(nums: Vec<i32>) -> i32 {
    let n = nums.len();
    let base = n + 1;
    let mut tree = Fenwick::new(n + base);
    tree.update(base, 1);
    let mut ans = 0i64;
    let mut s = 0i32;
    for x in nums {
        s += if x == 1 { 1 } else { -1 };
        ans += tree.query((s - 1 + base as i32) as usize);
        ans %= MOD;
        tree.update((s + base as i32) as usize, 1);
    }
    ans as i32
}

fn main() {
    println!("{}", subarrays_with_more_zeros_than_ones(vec![0, 1, 1, 0, 1]));
}

#[cfg(test)]
mod tests {
    use super::subarrays_with_more_zeros_than_ones;

    #[test]
    fn example_one() {
        assert_eq!(
            subarrays_with_more_zeros_than_ones(vec![0, 1, 1, 0, 1]),
            9
        );
    }

    #[test]
    fn example_two() {
        assert_eq!(subarrays_with_more_zeros_than_ones(vec![0]), 0);
    }
}
