/// LeetCode #3509 - Maximum Product of Subsequences With an Alternating Sum Equal to K
use std::collections::{HashMap, HashSet};

fn max_product(nums: Vec<i32>, k: i32, limit: i32) -> i32 {
    let total: i32 = nums.iter().sum();
    if k.abs() > total {
        return -1;
    }
    let limit = limit as i64;
    let mut dp: HashMap<(i32, u8), HashSet<i64>> = HashMap::new();
    for x in nums {
        let xl = x as i64;
        let mut new_dp = dp.clone();
        if xl <= limit {
            new_dp.entry((x, 1)).or_default().insert(xl);
        }
        for (&(s, par), products) in &dp {
            for &p in products {
                let np = p * xl;
                if np > limit {
                    continue;
                }
                let ns = s + if par == 0 { x } else { -x };
                new_dp.entry((ns, 1 - par)).or_default().insert(np);
            }
        }
        dp = new_dp;
    }
    let mut ans = -1i64;
    for par in [0u8, 1] {
        if let Some(ps) = dp.get(&(k, par)) {
            for &p in ps {
                ans = ans.max(p);
            }
        }
    }
    ans as i32
}

fn main() {
    println!("{}", max_product(vec![1, 2, 3], 2, 10));
}

#[cfg(test)]
mod tests {
    use super::max_product;

    #[test]
    fn example1() {
        assert_eq!(max_product(vec![1, 2, 3], 2, 10), 6);
    }

    #[test]
    fn example2() {
        assert_eq!(max_product(vec![0, 2, 3], -5, 12), -1);
    }

    #[test]
    fn example3() {
        assert_eq!(max_product(vec![2, 2, 3, 3], 0, 9), 9);
    }
}
