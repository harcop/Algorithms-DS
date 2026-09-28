/// LeetCode #3989 - Maximum Consistent Columns in a Grid
fn max_consistent_columns(grid: Vec<Vec<i32>>, limit: i32) -> i32 {
    let m = grid.len();
    let n = grid[0].len();
    let limit = limit as i64;
    let compatible = |a: usize, b: usize| -> bool {
        (0..m).all(|r| (grid[r][b] as i64 - grid[r][a] as i64).abs() <= limit)
    };
    let mut dp = vec![1i32; n];
    for j in 0..n {
        for i in 0..j {
            if compatible(i, j) {
                dp[j] = dp[j].max(dp[i] + 1);
            }
        }
    }
    dp.into_iter().max().unwrap()
}

fn main() {
    println!(
        "{}",
        max_consistent_columns(vec![vec![-2, 0, 3]], 2)
    );
}

#[cfg(test)]
mod tests {
    use super::max_consistent_columns;

    #[test]
    fn example1() {
        assert_eq!(max_consistent_columns(vec![vec![-2, 0, 3]], 2), 2);
    }

    #[test]
    fn example2() {
        assert_eq!(
            max_consistent_columns(vec![vec![1, -1, 1], vec![2, 2, 2]], 1),
            2
        );
    }

    #[test]
    fn example3() {
        assert_eq!(max_consistent_columns(vec![vec![-5, 5]], 9), 1);
    }
}
