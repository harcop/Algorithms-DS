/// LeetCode #1778 - Shortest Path in a Grid with Obstacles Elimination
use std::collections::VecDeque;

fn shortest_path(grid: Vec<Vec<i32>>, k: i32) -> i32 {
    let m = grid.len();
    let n = grid[0].len();
    if m == 1 && n == 1 {
        return 0;
    }
    let k = k as usize;
    let mut seen = vec![vec![vec![false; k + 1]; n]; m];
    let mut q = VecDeque::new();
    q.push_back((0, 0, 0, 0));
    seen[0][0][0] = true;
    let dirs = [(0, 1), (0, -1), (1, 0), (-1, 0)];
    while let Some((i, j, rem, steps)) = q.pop_front() {
        for (di, dj) in dirs {
            let ni = i as i32 + di;
            let nj = j as i32 + dj;
            if ni < 0 || nj < 0 || ni as usize >= m || nj as usize >= n {
                continue;
            }
            let ni = ni as usize;
            let nj = nj as usize;
            let next_rem = if grid[ni][nj] == 1 {
                if rem == k {
                    continue;
                }
                rem + 1
            } else {
                rem
            };
            if ni == m - 1 && nj == n - 1 {
                return steps + 1;
            }
            if !seen[ni][nj][next_rem] {
                seen[ni][nj][next_rem] = true;
                q.push_back((ni, nj, next_rem, steps + 1));
            }
        }
    }
    -1
}
fn main() {
    println!(
        "{}",
        shortest_path(
            vec![vec![0, 0, 0], vec![1, 1, 0], vec![0, 0, 0], vec![0, 1, 1], vec![0, 0, 0]],
            1,
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
                vec![vec![0, 0, 0], vec![1, 1, 0], vec![0, 0, 0], vec![0, 1, 1], vec![0, 0, 0]],
                1,
            ),
            6
        );
    }
    #[test]
    fn example_two() {
        assert_eq!(shortest_path(vec![vec![0, 1, 1], vec![1, 1, 1], vec![1, 0, 0]], 1), -1);
    }
}
