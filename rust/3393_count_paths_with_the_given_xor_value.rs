/// LeetCode #3393 - Count Paths With the Given XOR Value
fn count_paths_with_xor_value(grid: Vec<Vec<i32>>, k: i32) -> i32 {
    const MOD: i32 = 1_000_000_007;
    let m = grid.len();
    let n = grid[0].len();
    let mut dp = vec![vec![vec![0i32; 16]; n]; m];
    dp[0][0][grid[0][0] as usize] = 1;
    for i in 0..m {
        for j in 0..n {
            for xor_val in 0..16 {
                let ways = dp[i][j][xor_val];
                if ways == 0 {
                    continue;
                }
                if i + 1 < m {
                    let nx = xor_val ^ grid[i + 1][j] as usize;
                    dp[i + 1][j][nx] = (dp[i + 1][j][nx] + ways) % MOD;
                }
                if j + 1 < n {
                    let nx = xor_val ^ grid[i][j + 1] as usize;
                    dp[i][j + 1][nx] = (dp[i][j + 1][nx] + ways) % MOD;
                }
            }
        }
    }
    dp[m - 1][n - 1][k as usize]
}

fn main() {
    println!(
        "{}",
        count_paths_with_xor_value(vec![vec![2, 1, 5], vec![7, 10, 0], vec![12, 6, 4]], 11)
    );
}

#[cfg(test)]
mod tests {
    use super::count_paths_with_xor_value;

    #[test]
    fn example1() {
        assert_eq!(
            count_paths_with_xor_value(vec![vec![2, 1, 5], vec![7, 10, 0], vec![12, 6, 4]], 11),
            3
        );
    }

    #[test]
    fn example2() {
        assert_eq!(
            count_paths_with_xor_value(
                vec![vec![1, 3, 3, 3], vec![0, 3, 3, 2], vec![3, 0, 1, 1]],
                2
            ),
            5
        );
    }

    #[test]
    fn example3() {
        assert_eq!(
            count_paths_with_xor_value(
                vec![vec![1, 1, 1, 2], vec![3, 0, 3, 2], vec![3, 0, 2, 2]],
                10
            ),
            0
        );
    }
}
