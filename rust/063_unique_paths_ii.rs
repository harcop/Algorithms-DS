/// LeetCode #63 - Unique Paths II
fn unique_paths_with_obstacles(obstacle_grid: Vec<Vec<i32>>) -> i32 {
    let m = obstacle_grid.len();
    let n = obstacle_grid[0].len();
    let mut dp = vec![0; n];
    dp[0] = if obstacle_grid[0][0] == 1 { 0 } else { 1 };

    for row in obstacle_grid.iter().take(m) {
        for j in 0..n {
            if row[j] == 1 {
                dp[j] = 0;
            } else if j > 0 {
                dp[j] += dp[j - 1];
            }
        }
    }
    dp[n - 1]
}

fn main() {
    println!("{}", unique_paths_with_obstacles(vec![vec![0,0,0],vec![0,1,0],vec![0,0,0]]));
}

#[cfg(test)]
mod tests {
    use super::unique_paths_with_obstacles;
    #[test]
    fn example_one() {
        assert_eq!(unique_paths_with_obstacles(vec![vec![0,0,0],vec![0,1,0],vec![0,0,0]]), 2);
    }
    #[test]
    fn example_two() {
        assert_eq!(unique_paths_with_obstacles(vec![vec![0,1],vec![0,0]]), 1);
    }
}
