/// LeetCode #64 - Minimum Path Sum
fn min_path_sum(grid: Vec<Vec<i32>>) -> i32 {
    let m = grid.len();
    let n = grid[0].len();
    let mut dp = vec![i32::MAX; n];
    dp[0] = 0;

    for row in grid.iter().take(m) {
        dp[0] += row[0];
        for j in 1..n {
            dp[j] = dp[j].min(dp[j - 1]) + row[j];
        }
    }
    dp[n - 1]
}

fn main() {
    println!("{}", min_path_sum(vec![vec![1,3,1],vec![1,5,1],vec![4,2,1]]));
}

#[cfg(test)]
mod tests {
    use super::min_path_sum;
    #[test]
    fn example_one() {
        assert_eq!(min_path_sum(vec![vec![1,3,1],vec![1,5,1],vec![4,2,1]]), 7);
    }
    #[test]
    fn example_two() {
        assert_eq!(min_path_sum(vec![vec![1,2,3],vec![4,5,6]]), 12);
    }
}
