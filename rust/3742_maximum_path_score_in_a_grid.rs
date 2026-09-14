/// LeetCode #3742 - Maximum Path Score in a Grid
fn max_path_score(grid: Vec<Vec<i32>>, k: i32) -> i32 {
    let m = grid.len();
    let n = grid[0].len();
    let k = k as usize;
    let mut dp = vec![vec![vec![-1i32; k + 1]; n]; m];
    dp[0][0][0] = 0;
    for i in 0..m {
        for j in 0..n {
            for c in 0..=k {
                let cur = dp[i][j][c];
                if cur < 0 {
                    continue;
                }
                if i + 1 < m {
                    let v = grid[i + 1][j];
                    let nc = c + if v == 0 { 0 } else { 1 };
                    if nc <= k {
                        dp[i + 1][j][nc] = dp[i + 1][j][nc].max(cur + v);
                    }
                }
                if j + 1 < n {
                    let v = grid[i][j + 1];
                    let nc = c + if v == 0 { 0 } else { 1 };
                    if nc <= k {
                        dp[i][j + 1][nc] = dp[i][j + 1][nc].max(cur + v);
                    }
                }
            }
        }
    }
    *dp[m - 1][n - 1].iter().max().unwrap()
}

fn main() {
    println!("{}", max_path_score(vec![vec![0, 1], vec![2, 0]], 1));
}

#[cfg(test)]
mod tests {
    use super::max_path_score;

    #[test]
    fn example1() {
        assert_eq!(max_path_score(vec![vec![0, 1], vec![2, 0]], 1), 2);
    }

    #[test]
    fn example2() {
        assert_eq!(max_path_score(vec![vec![0, 1], vec![1, 2]], 1), -1);
    }
}
