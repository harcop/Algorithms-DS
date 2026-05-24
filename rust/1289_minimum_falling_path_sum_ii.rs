/// LeetCode #1289 - Minimum Falling Path Sum II
fn min_falling_path_sum(grid: Vec<Vec<i32>>) -> i32 {
    let n = grid.len();
    if n == 0 {
        return 0;
    }
    let mut dp = grid[0].clone();
    for i in 1..n {
        let mut ndp = vec![i32::MAX; n];
        for j in 0..n {
            for k in 0..n {
                if k != j {
                    ndp[j] = ndp[j].min(dp[k] + grid[i][j]);
                }
            }
        }
        dp = ndp;
    }
    *dp.iter().min().unwrap()
}

fn main() {
    println!(
        "{}",
        min_falling_path_sum(vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]])
    );
}

#[cfg(test)]
mod tests {
    use super::min_falling_path_sum;

    #[test]
    fn example_one() {
        assert_eq!(
            min_falling_path_sum(vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]]),
            13
        );
    }

    #[test]
    fn example_two() {
        assert_eq!(min_falling_path_sum(vec![vec![7]]), 7);
    }
}
