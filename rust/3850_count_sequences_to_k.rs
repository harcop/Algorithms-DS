/// LeetCode #3850 - Count Sequences to K
use std::collections::HashMap;

fn gcd(mut a: i64, mut b: i64) -> i64 {
    while b != 0 {
        let t = a % b;
        a = b;
        b = t;
    }
    a
}

fn count_sequences(nums: Vec<i32>, k: i64) -> i32 {
    let n = nums.len();
    let mut f: HashMap<(usize, i64, i64), i32> = HashMap::new();
    fn dfs(
        i: usize,
        p: i64,
        q: i64,
        nums: &[i32],
        n: usize,
        k: i64,
        f: &mut HashMap<(usize, i64, i64), i32>,
    ) -> i32 {
        if i == n {
            return if p == k && q == 1 { 1 } else { 0 };
        }
        if let Some(&v) = f.get(&(i, p, q)) {
            return v;
        }
        let mut res = dfs(i + 1, p, q, nums, n, k, f);
        let x = nums[i] as i64;
        let g1 = gcd(p * x, q);
        res += dfs(i + 1, (p * x) / g1, q / g1, nums, n, k, f);
        let g2 = gcd(p, q * x);
        res += dfs(i + 1, p / g2, (q * x) / g2, nums, n, k, f);
        f.insert((i, p, q), res);
        res
    }
    dfs(0, 1, 1, &nums, n, k, &mut f)
}

fn main() {
    println!("{}", count_sequences(vec![2, 3, 2], 6));
}

#[cfg(test)]
mod tests {
    use super::count_sequences;

    #[test]
    fn example1() {
        assert_eq!(count_sequences(vec![2, 3, 2], 6), 2);
    }

    #[test]
    fn example2() {
        assert_eq!(count_sequences(vec![4, 6, 3], 2), 2);
    }

    #[test]
    fn example3() {
        assert_eq!(count_sequences(vec![1, 5], 1), 3);
    }
}
