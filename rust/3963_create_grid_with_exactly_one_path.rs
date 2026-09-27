/// LeetCode #3963 - Create Grid With Exactly One Path
fn create_grid(m: i32, n: i32) -> Vec<String> {
    let m = m as usize;
    let n = n as usize;
    let mut g = vec![vec![b'#'; n]; m];
    g[0] = vec![b'.'; n];
    for row in &mut g {
        row[n - 1] = b'.';
    }
    g.into_iter()
        .map(|row| String::from_utf8(row).unwrap())
        .collect()
}

fn main() {
    println!("{:?}", create_grid(2, 3));
}

#[cfg(test)]
mod tests {
    use super::create_grid;

    fn path_count(grid: &[String]) -> i64 {
        let m = grid.len();
        let n = grid[0].len();
        let g: Vec<Vec<u8>> = grid.iter().map(|s| s.as_bytes().to_vec()).collect();
        let mut dp = vec![vec![0i64; n]; m];
        for i in 0..m {
            for j in 0..n {
                if g[i][j] != b'.' {
                    continue;
                }
                if i == 0 && j == 0 {
                    dp[i][j] = 1;
                } else {
                    if i > 0 {
                        dp[i][j] += dp[i - 1][j];
                    }
                    if j > 0 {
                        dp[i][j] += dp[i][j - 1];
                    }
                }
            }
        }
        dp[m - 1][n - 1]
    }

    #[test]
    fn example1() {
        let g = create_grid(2, 3);
        assert_eq!(path_count(&g), 1);
    }

    #[test]
    fn example2() {
        let g = create_grid(3, 3);
        assert_eq!(path_count(&g), 1);
    }

    #[test]
    fn example3() {
        assert_eq!(create_grid(1, 4), vec!["....".to_string()]);
    }
}
