/// LeetCode #2435 - Paths in Matrix Whose Sum Is Divisible by K
fn number_of_paths(grid: Vec<Vec<i32>>, k: i32) -> i32 {
    const MOD: i64 = 1_000_000_007;

    let rows = grid.len();
    let cols = grid[0].len();
    let k = k as usize;
    let mut dp = vec![vec![vec![0i64; k]; cols]; rows];
    dp[0][0][grid[0][0] as usize % k] = 1;

    for row in 0..rows {
        for col in 0..cols {
            if row == 0 && col == 0 {
                continue;
            }
            let value = grid[row][col] as usize % k;
            for remainder in 0..k {
                let next = (remainder + value) % k;
                if row > 0 {
                    dp[row][col][next] = (dp[row][col][next] + dp[row - 1][col][remainder]) % MOD;
                }
                if col > 0 {
                    dp[row][col][next] = (dp[row][col][next] + dp[row][col - 1][remainder]) % MOD;
                }
            }
        }
    }

    dp[rows - 1][cols - 1][0] as i32
}

fn main() {
    println!(
        "{}",
        number_of_paths(vec![vec![5, 2, 4], vec![3, 0, 5], vec![0, 7, 2]], 3)
    );
}

#[cfg(test)]
mod tests {
    use super::number_of_paths;

    #[test]
    fn example_one() {
        assert_eq!(
            number_of_paths(vec![vec![5, 2, 4], vec![3, 0, 5], vec![0, 7, 2]], 3),
            2
        );
    }

    #[test]
    fn example_two() {
        assert_eq!(number_of_paths(vec![vec![0, 0]], 5), 1);
    }
}
