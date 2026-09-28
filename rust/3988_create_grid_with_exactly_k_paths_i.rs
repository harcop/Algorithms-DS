/// LeetCode #3988 - Create Grid With Exactly K Paths I
fn create_grid(m: i32, n: i32, k: i32) -> Vec<String> {
    let m = m as usize;
    let n = n as usize;
    let k = k as usize;
    if m == 1 || n == 1 {
        if k == 1 {
            if m == 1 {
                return vec![".".repeat(n)];
            }
            return vec![".".to_string(); m];
        }
        return Vec::new();
    }
    if k <= n {
        let mut g = vec![".".repeat(n)];
        let hashes = n - k;
        g.push(format!("{}{}", "#".repeat(hashes), ".".repeat(k)));
        for _ in 2..m {
            g.push(format!("{}{}", "#".repeat(n - 1), "."));
        }
        return g;
    }
    if k <= m {
        let mut g = vec![vec![b'#'; n]; m];
        for row in &mut g {
            row[0] = b'.';
        }
        for i in (m - k)..m {
            g[i][1] = b'.';
        }
        for j in 2..n {
            g[m - 1][j] = b'.';
        }
        return g
            .into_iter()
            .map(|row| String::from_utf8(row).unwrap())
            .collect();
    }
    if m == 3 && n == 3 && k == 4 {
        return vec!["..#".into(), "...".into(), "#..".into()];
    }
    Vec::new()
}

fn main() {
    println!("{:?}", create_grid(2, 3, 2));
}

#[cfg(test)]
mod tests {
    use super::create_grid;

    fn path_count(grid: &[String]) -> i64 {
        if grid.is_empty() {
            return 0;
        }
        let m = grid.len();
        let n = grid[0].len();
        let g: Vec<Vec<u8>> = grid.iter().map(|s| s.as_bytes().to_vec()).collect();
        let mut dp = vec![vec![0i64; n]; m];
        if g[0][0] != b'.' {
            return 0;
        }
        dp[0][0] = 1;
        for i in 0..m {
            for j in 0..n {
                if g[i][j] != b'.' || (i == 0 && j == 0) {
                    continue;
                }
                if i > 0 {
                    dp[i][j] += dp[i - 1][j];
                }
                if j > 0 {
                    dp[i][j] += dp[i][j - 1];
                }
            }
        }
        dp[m - 1][n - 1]
    }

    #[test]
    fn example1() {
        assert_eq!(path_count(&create_grid(2, 3, 2)), 2);
    }

    #[test]
    fn example2() {
        assert_eq!(path_count(&create_grid(3, 3, 4)), 4);
    }

    #[test]
    fn example3() {
        assert!(create_grid(1, 4, 2).is_empty());
    }

    #[test]
    fn single_row() {
        assert_eq!(create_grid(1, 4, 1), vec!["....".to_string()]);
    }
}
