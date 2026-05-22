/// LeetCode #1210 - Minimum Moves to Reach Target with Rotations
use std::collections::VecDeque;

fn minimum_moves(grid: Vec<Vec<i32>>) -> i32 {
    let n = grid.len();
    if n == 0 {
        return -1;
    }
    let mut dist = vec![vec![[i32::MAX; 2]; n]; n];
    let mut q = VecDeque::from([(0usize, 0usize, 0u8, 0i32)]);
    dist[0][0][0] = 0;
    while let Some((r, c, p, d)) = q.pop_front() {
        if r == n - 1 && c == n - 2 && p == 0 {
            return d;
        }
        if p == 0 {
            if c + 2 < n && grid[r][c + 2] == 0 && d + 1 < dist[r][c + 1][0] {
                dist[r][c + 1][0] = d + 1;
                q.push_back((r, c + 1, 0, d + 1));
            }
            if r + 1 < n
                && c + 1 < n
                && grid[r + 1][c] == 0
                && grid[r + 1][c + 1] == 0
                && d + 1 < dist[r + 1][c][0]
            {
                dist[r + 1][c][0] = d + 1;
                q.push_back((r + 1, c, 0, d + 1));
            }
            if r + 1 < n
                && c + 1 < n
                && grid[r + 1][c + 1] == 0
                && d + 1 < dist[r][c][1]
            {
                dist[r][c][1] = d + 1;
                q.push_back((r, c, 1, d + 1));
            }
        } else {
            if r + 2 < n && grid[r + 2][c] == 0 && d + 1 < dist[r + 1][c][1] {
                dist[r + 1][c][1] = d + 1;
                q.push_back((r + 1, c, 1, d + 1));
            }
            if c + 1 < n
                && grid[r][c + 1] == 0
                && grid[r + 1][c + 1] == 0
                && d + 1 < dist[r][c + 1][1]
            {
                dist[r][c + 1][1] = d + 1;
                q.push_back((r, c + 1, 1, d + 1));
            }
            if c + 1 < n
                && grid[r + 1][c] == 0
                && grid[r + 1][c + 1] == 0
                && d + 1 < dist[r][c][0]
            {
                dist[r][c][0] = d + 1;
                q.push_back((r, c, 0, d + 1));
            }
        }
    }
    -1
}

fn main() {
    println!(
        "{}",
        minimum_moves(vec![
            vec![0, 0, 0, 0, 0, 1],
            vec![1, 1, 0, 0, 1, 0],
            vec![0, 0, 0, 0, 1, 1],
            vec![0, 0, 1, 0, 1, 0],
            vec![0, 1, 1, 0, 0, 0],
            vec![0, 1, 1, 0, 0, 0],
        ])
    );
}

#[cfg(test)]
mod tests {
    use super::minimum_moves;

    #[test]
    fn example_one() {
        assert_eq!(
            minimum_moves(vec![
                vec![0, 0, 0, 0, 0, 1],
                vec![1, 1, 0, 0, 1, 0],
                vec![0, 0, 0, 0, 1, 1],
                vec![0, 0, 1, 0, 1, 0],
                vec![0, 1, 1, 0, 0, 0],
                vec![0, 1, 1, 0, 0, 0],
            ]),
            11
        );
    }

    #[test]
    fn example_two() {
        assert_eq!(
            minimum_moves(vec![
                vec![0, 0, 1, 1, 1, 1],
                vec![0, 0, 0, 0, 1, 1],
                vec![1, 1, 0, 0, 0, 1],
                vec![1, 1, 1, 0, 0, 1],
                vec![1, 1, 1, 0, 0, 1],
                vec![1, 1, 1, 0, 0, 0],
            ]),
            9
        );
    }
}
