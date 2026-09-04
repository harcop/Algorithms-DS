/// LeetCode #486 - Predict the Winner
fn predict_the_winner(nums: Vec<i32>) -> bool {
    let n = nums.len();
    let mut dp = vec![vec![0i32; n]; n];
    for i in 0..n {
        dp[i][i] = nums[i];
    }
    for len in 2..=n {
        for i in 0..=n - len {
            let j = i + len - 1;
            dp[i][j] = (nums[i] - dp[i + 1][j]).max(nums[j] - dp[i][j - 1]);
        }
    }
    dp[0][n - 1] >= 0
}

fn main() {
    println!("{}", predict_the_winner(vec![1, 5, 2]));
}

#[cfg(test)]
mod tests {
    use super::predict_the_winner;

    #[test]
    fn example_one() {
        assert!(!predict_the_winner(vec![1, 5, 2]));
    }

    #[test]
    fn example_two() {
        assert!(predict_the_winner(vec![1, 5, 233, 7]));
    }
}
