/// LeetCode #3665 - Twisted Mirror Path Count
fn unique_paths(grid: Vec<Vec<i32>>) -> i32 {
    const MOD: i64 = 1_000_000_007;
    let n = grid.len();
    let m = grid[0].len();
    let mut go = vec![vec![[(0usize, 0usize); 2]; m]; n];
    for i in (0..n).rev() {
        for j in (0..m).rev() {
            if grid[i][j] == 0 {
                go[i][j][0] = (i, j);
                go[i][j][1] = (i, j);
            } else {
                // dir 0 = down attempt, dir 1 = right attempt (after landing on mirror)
                go[i][j][1] = if j + 1 < m {
                    go[i][j + 1][0]
                } else {
                    (i, j + 1)
                };
                go[i][j][0] = if i + 1 < n {
                    go[i + 1][j][1]
                } else {
                    (i + 1, j)
                };
            }
        }
    }
    let mut dp = vec![vec![0i64; m]; n];
    dp[0][0] = 1;
    let dy = [1usize, 0];
    let dx = [0usize, 1];
    for i in 0..n {
        for j in 0..m {
            if dp[i][j] == 0 {
                continue;
            }
            for dir in 0..2 {
                let ni = i + dy[dir];
                let nj = j + dx[dir];
                if ni < n && nj < m {
                    let (ny, nx) = go[ni][nj][1 - dir];
                    if ny < n && nx < m {
                        dp[ny][nx] = (dp[ny][nx] + dp[i][j]) % MOD;
                    }
                }
            }
        }
    }
    dp[n - 1][m - 1] as i32
}

fn main() {
    println!("{}", unique_paths(vec![vec![0, 1, 0], vec![0, 0, 1], vec![1, 0, 0]]));
}

#[cfg(test)]
mod tests {
    use super::unique_paths;

    #[test]
    fn example1() {
        assert_eq!(
            unique_paths(vec![vec![0, 1, 0], vec![0, 0, 1], vec![1, 0, 0]]),
            5
        );
    }

    #[test]
    fn example2() {
        assert_eq!(unique_paths(vec![vec![0, 0], vec![0, 0]]), 2);
    }

    #[test]
    fn example3() {
        assert_eq!(unique_paths(vec![vec![0, 1, 1], vec![1, 1, 0]]), 1);
    }
}
