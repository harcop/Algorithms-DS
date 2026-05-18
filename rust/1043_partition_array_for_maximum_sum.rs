/// LeetCode #1043 - Partition Array for Maximum Sum
fn max_sum_after_partitioning(arr: Vec<i32>, k: i32) -> i32 {
    let k = k as usize;
    let n = arr.len();
    let mut dp = vec![0i32; n + 1];
    for i in 1..=n {
        let mut best = arr[i - 1];
        for j in 1..=k.min(i) {
            best = best.max(arr[i - j]);
            dp[i] = dp[i].max(dp[i - j] + best * j as i32);
        }
    }
    dp[n]
}

fn main() {
    println!("{}", max_sum_after_partitioning(vec![1, 15, 7, 9, 2, 5, 10], 3));
}

#[cfg(test)]
mod tests {
    use super::max_sum_after_partitioning;

    #[test]
    fn example_one() {
        assert_eq!(max_sum_after_partitioning(vec![1, 15, 7, 9, 2, 5, 10], 3), 84);
    }

    #[test]
    fn example_two() {
        assert_eq!(max_sum_after_partitioning(vec![1, 4, 1, 5, 7, 3, 6, 1, 9, 9, 3], 4), 83);
    }
}
