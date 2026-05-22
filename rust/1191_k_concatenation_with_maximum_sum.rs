/// LeetCode #1191 - K-Concatenation With Maximum Sum
const MOD: i64 = 1_000_000_007;

fn kadane(nums: &[i32]) -> i64 {
    let mut best = 0i64;
    let mut cur = 0i64;
    for &x in nums {
        cur = (cur + x as i64).max(x as i64);
        best = best.max(cur);
    }
    best
}

fn k_concatenation_max_sum(nums: Vec<i32>, k: i32) -> i32 {
    let k = k as i64;
    let n = nums.len();
    if n == 0 {
        return 0;
    }
    let total: i64 = nums.iter().map(|&x| x as i64).sum();
    let max_sub = kadane(&nums);
    if k == 1 {
        return (max_sub % MOD) as i32;
    }
    let mut prefix = 0i64;
    let mut best_prefix = i64::MIN;
    let mut acc = 0i64;
    for &x in &nums {
        acc += x as i64;
        prefix = prefix.max(acc);
        best_prefix = best_prefix.max(prefix);
    }
    let mut suffix = 0i64;
    let mut best_suffix = i64::MIN;
    acc = 0;
    for &x in nums.iter().rev() {
        acc += x as i64;
        suffix = suffix.max(acc);
        best_suffix = best_suffix.max(suffix);
    }
    let mut ans = max_sub.max(best_prefix + best_suffix);
    if total > 0 {
        ans = ans.max(best_prefix + best_suffix + (k - 2) * total);
    }
    (ans % MOD) as i32
}

fn main() {
    println!("{}", k_concatenation_max_sum(vec![1, 2], 3));
}

#[cfg(test)]
mod tests {
    use super::k_concatenation_max_sum;

    #[test]
    fn example_one() {
        assert_eq!(k_concatenation_max_sum(vec![1, 2], 3), 9);
    }

    #[test]
    fn example_two() {
        assert_eq!(k_concatenation_max_sum(vec![1, -2, 1], 5), 2);
    }

    #[test]
    fn example_three() {
        assert_eq!(k_concatenation_max_sum(vec![-1, -2], 7), 0);
    }

    #[test]
    fn large_k() {
        assert_eq!(k_concatenation_max_sum(vec![10000], 100_000), 1_000_000_000);
    }
}
