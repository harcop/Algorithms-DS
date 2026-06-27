/// LeetCode #2123 - Minimum Operations to Remove Adjacent Ones in Matrix
fn minimum_operations(grid: Vec<Vec<i32>>) -> i32 {
    let m = grid.len();
    let n = grid[0].len();
    let mut matched_to = vec![None; m * n];
    let dirs = [(1i32, 0i32), (-1, 0), (0, 1), (0, -1)];

    fn dfs(
        r: usize,
        c: usize,
        grid: &[Vec<i32>],
        matched_to: &mut [Option<(usize, usize)>],
        seen: &mut [bool],
        dirs: &[(i32, i32)],
    ) -> bool {
        let m = grid.len() as i32;
        let n = grid[0].len() as i32;

        for &(dr, dc) in dirs {
            let nr = r as i32 + dr;
            let nc = c as i32 + dc;
            if nr < 0 || nr >= m || nc < 0 || nc >= n || grid[nr as usize][nc as usize] == 0 {
                continue;
            }

            let v = nr as usize * n as usize + nc as usize;
            if seen[v] {
                continue;
            }
            seen[v] = true;

            if matched_to[v].is_none()
                || dfs(
                    matched_to[v].unwrap().0,
                    matched_to[v].unwrap().1,
                    grid,
                    matched_to,
                    seen,
                    dirs,
                )
            {
                matched_to[v] = Some((r, c));
                return true;
            }
        }

        false
    }

    let mut ans = 0;
    for r in 0..m {
        for c in 0..n {
            if grid[r][c] == 1 && (r + c) % 2 == 0 {
                let mut seen = vec![false; m * n];
                if dfs(r, c, &grid, &mut matched_to, &mut seen, &dirs) {
                    ans += 1;
                }
            }
        }
    }

    ans
}

fn main() {
    println!(
        "{}",
        minimum_operations(vec![vec![1, 1, 0], vec![0, 1, 1], vec![1, 1, 1]])
    );
}

#[cfg(test)]
mod tests {
    use super::minimum_operations;

    #[test]
    fn example_one() {
        assert_eq!(
            minimum_operations(vec![vec![1, 1, 0], vec![0, 1, 1], vec![1, 1, 1]]),
            3
        );
    }

    #[test]
    fn example_two() {
        assert_eq!(
            minimum_operations(vec![vec![0, 0, 0], vec![0, 0, 0], vec![0, 0, 0]]),
            0
        );
    }

    #[test]
    fn example_three() {
        assert_eq!(minimum_operations(vec![vec![0, 1], vec![1, 0]]), 0);
    }
}
