/// LeetCode #3990 - Create Grid With Exactly K Paths II
fn create_grid(k: i32) -> Vec<String> {
    let k = k as usize;
    let h = (31 - (k as u32).leading_zeros()) as usize;
    let rows = 2 * h + 1;
    let cols = 2 * h + 3;
    let mut g = vec![vec![b'#'; cols]; rows];
    for i in 0..h {
        let r = 2 * i;
        let c = 2 * i;
        for (rr, cc) in [
            (r, c),
            (r, c + 1),
            (r, c + 2),
            (r + 1, c + 2),
            (r + 2, c + 2),
            (r + 1, c),
            (r + 2, c),
            (r + 2, c + 1),
        ] {
            g[rr][cc] = b'.';
        }
    }
    g[2 * h][2 * h] = b'.';
    let col_c = 2 * h + 2;
    for i in 0..=h {
        if (k >> i) & 1 == 1 {
            let r = 2 * i;
            let start_c = if i < h { 2 * i + 2 } else { 2 * h };
            for c in start_c..=col_c {
                g[r][c] = b'.';
            }
            for rr in r..=2 * h {
                g[rr][col_c] = b'.';
            }
        }
    }
    g.into_iter()
        .map(|row| String::from_utf8(row).unwrap())
        .collect()
}

fn main() {
    println!("{:?}", create_grid(2));
}

#[cfg(test)]
mod tests {
    use super::create_grid;

    fn path_count(grid: &[String]) -> i64 {
        let m = grid.len();
        let n = grid[0].len();
        let g: Vec<&[u8]> = grid.iter().map(|s| s.as_bytes()).collect();
        let mut dp = vec![vec![0i64; n]; m];
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
        let g = create_grid(2);
        assert!(g.len() <= 25 && g[0].len() <= 25);
        assert_eq!(path_count(&g), 2);
    }

    #[test]
    fn example2() {
        assert_eq!(path_count(&create_grid(3)), 3);
    }

    #[test]
    fn powers_and_thousand() {
        for k in [1, 4, 5, 7, 16, 100, 511, 512, 1000] {
            let g = create_grid(k);
            assert!(g.len() <= 25 && g[0].len() <= 25);
            assert_eq!(path_count(&g), k as i64, "k={k}");
        }
    }
}
