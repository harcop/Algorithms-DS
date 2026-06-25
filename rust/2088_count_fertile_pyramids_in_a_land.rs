/// LeetCode #2088 - Count Fertile Pyramids in a Land
fn count_pyramids(grid: Vec<Vec<i32>>) -> i32 {
    let mut reversed = grid.clone();
    reversed.reverse();
    count_one_direction(&grid) + count_one_direction(&reversed)
}

fn count_one_direction(grid: &[Vec<i32>]) -> i32 {
    let m = grid.len();
    let n = grid[0].len();
    let mut dp = vec![vec![0; n]; m];
    let mut ans = 0;

    for i in (0..m).rev() {
        for j in 0..n {
            if grid[i][j] == 0 {
                continue;
            }
            dp[i][j] = 1;
            if i + 1 < m && j > 0 && j + 1 < n {
                dp[i][j] += dp[i + 1][j - 1].min(dp[i + 1][j]).min(dp[i + 1][j + 1]);
            }
            ans += dp[i][j] - 1;
        }
    }

    ans
}

fn main() {
    println!("{}", count_pyramids(vec![vec![0, 1, 1, 0], vec![1, 1, 1, 1]]));
}

#[cfg(test)]
mod tests {
    use super::count_pyramids;

    #[test]
    fn example_one() {
        assert_eq!(count_pyramids(vec![vec![0, 1, 1, 0], vec![1, 1, 1, 1]]), 2);
    }

    #[test]
    fn example_two() {
        assert_eq!(count_pyramids(vec![vec![1, 1, 1], vec![1, 1, 1]]), 2);
    }

    #[test]
    fn example_three() {
        assert_eq!(
            count_pyramids(vec![vec![1, 0, 1], vec![0, 0, 0], vec![1, 0, 1]]),
            0
        );
    }
}
