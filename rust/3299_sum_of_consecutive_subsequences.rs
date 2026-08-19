/// LeetCode #3299 - Sum of Consecutive Subsequences
use std::collections::HashMap;

fn get_sum(mut nums: Vec<i32>) -> i32 {
    const MOD: i64 = 1_000_000_007;
    fn calc(nums: &[i32]) -> i64 {
        let n = nums.len();
        let mut left = vec![0i64; n];
        let mut right = vec![0i64; n];
        let mut cnt: HashMap<i32, i64> = HashMap::new();
        for i in 1..n {
            let prev = nums[i - 1];
            let add = 1 + cnt.get(&(prev - 1)).copied().unwrap_or(0);
            *cnt.entry(prev).or_insert(0) += add;
            left[i] = cnt.get(&(nums[i] - 1)).copied().unwrap_or(0);
        }
        cnt.clear();
        for i in (0..n.saturating_sub(1)).rev() {
            let nxt = nums[i + 1];
            let add = 1 + cnt.get(&(nxt + 1)).copied().unwrap_or(0);
            *cnt.entry(nxt).or_insert(0) += add;
            right[i] = cnt.get(&(nums[i] + 1)).copied().unwrap_or(0);
        }
        nums.iter()
            .zip(left.iter().zip(right.iter()))
            .map(|(&x, (&l, &r))| (l + r + l * r) % MOD * (x as i64) % MOD)
            .sum::<i64>()
            % MOD
    }
    let x = calc(&nums);
    nums.reverse();
    let y = calc(&nums);
    let s: i64 = nums.iter().map(|&v| v as i64).sum::<i64>() % MOD;
    ((x + y + s) % MOD) as i32
}

fn main() {
    println!("{}", get_sum(vec![1, 2]));
}

#[cfg(test)]
mod tests {
    use super::get_sum;

    #[test]
    fn example1() {
        assert_eq!(get_sum(vec![1, 2]), 6);
    }

    #[test]
    fn example2() {
        assert_eq!(get_sum(vec![1, 4, 2, 3]), 31);
    }
}
