/// LeetCode #1293 - Shortest Path in a Grid with Obstacles Elimination
use std::collections::VecDeque;

fn shortest_path(grid: Vec<Vec<i32>>, k: i32) -> i32 {
    let m = grid.len();
    if m == 0 {
        return -1;
    }
    let n = grid[0].len();
    if m == 1 && n == 1 {
        return 0;
    }
    let k = k as usize;
    let mut seen = vec![vec![vec![false; k + 1]; n]; m];
    let mut q = VecDeque::new();
    q.push_back((0, 0, 0, 0));
    seen[0][0][0] = true;
    let dirs = [(0, 1), (1, 0), (0, -1), (-1, 0)];
    while let Some((r, c, obs, steps)) = q.pop_front() {
        for (dr, dc) in dirs {
            let nr = r as i32 + dr;
            let nc = c as i32 + dc;
            if nr < 0 || nc < 0 || nr >= m as i32 || nc >= n as i32 {
                continue;
            }
            let nr = nr as usize;
            let nc = nc as usize;
            if nr == m - 1 && nc == n - 1 {
                return steps + 1;
            }
            let nobs = obs + grid[nr][nc] as usize;
            if nobs <= k && !seen[nr][nc][nobs] {
                seen[nr][nc][nobs] = true;
                q.push_back((nr, nc, nobs, steps + 1));
            }
        }
    }
    -1
}

fn main() {
    println!(
        "{}",
        shortest_path(
            vec![
                vec![0, 0, 0],
                vec![1, 1, 0],
                vec![0, 0, 0],
                vec![0, 1, 1],
                vec![0, 0, 0],
            ],
            1
        )
    );
}

#[cfg(test)]
mod tests {
    use super::shortest_path;

    #[test]
    fn example_one() {
        assert_eq!(
            shortest_path(
                vec![
                    vec![0, 0, 0],
                    vec![1, 1, 0],
                    vec![0, 0, 0],
                    vec![0, 1, 1],
                    vec![0, 0, 0],
                ],
                1
            ),
            6
        );
    }

    #[test]
    fn example_two() {
        assert_eq!(shortest_path(vec![vec![0, 1, 1], vec![1, 1, 1], vec![1, 0, 0]], 1), -1);
    }
}
