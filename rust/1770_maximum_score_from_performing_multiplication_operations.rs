/// LeetCode #1770 - Maximum Score from Performing Multiplication Operations
fn maximum_score(nums: Vec<i32>, multipliers: Vec<i32>) -> i32 {
    let n = nums.len();
    let m = multipliers.len();
    let mut dp = vec![vec![0i32; m + 1]; m + 1];
    for left in 0..=m {
        dp[left][m] = 0;
    }
    for op in (0..m).rev() {
        for left in 0..=op {
            let right = op - left;
            let score_l = nums[left] * multipliers[op] + dp[left + 1][op + 1];
            let score_r = nums[n - 1 - right] * multipliers[op] + dp[left][op + 1];
            dp[left][op] = score_l.max(score_r);
        }
    }
    dp[0][0]
}
fn main() {
    println!(
        "{}",
        maximum_score(vec![-5, -3, -1, 0, 1, 2], vec![-5, -3, -1, 0, 1, 2])
    );
}
#[cfg(test)]
mod tests {
    use super::maximum_score;
    #[test]
    fn example_one() {
        assert_eq!(
            maximum_score(vec![-5, -3, -1, 0, 1, 2], vec![-5, -3, -1, 0, 1, 2]),
            102
        );
    }
    #[test]
    fn example_two() {
        assert_eq!(maximum_score(vec![-1, 0, 1], vec![1, 1, 1]), 3);
    }
}
