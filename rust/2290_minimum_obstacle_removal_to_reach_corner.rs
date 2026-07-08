/// LeetCode #2290 - Minimum Obstacle Removal to Reach Corner
use std::collections::VecDeque;

fn minimum_obstacles(grid: Vec<Vec<i32>>) -> i32 {
    let m = grid.len();
    let n = grid[0].len();
    let mut dist = vec![i32::MAX; m * n];
    let mut dq: VecDeque<(usize, usize)> = VecDeque::new();

    dist[0] = 0;
    dq.push_front((0, 0));
    let dirs = [(1i32, 0i32), (-1, 0), (0, 1), (0, -1)];

    while let Some((r, c)) = dq.pop_front() {
        let d = dist[r * n + c];
        if r == m - 1 && c == n - 1 {
            return d;
        }
        for (dr, dc) in dirs {
            let nr = r as i32 + dr;
            let nc = c as i32 + dc;
            if nr < 0 || nc < 0 || nr >= m as i32 || nc >= n as i32 {
                continue;
            }
            let nr = nr as usize;
            let nc = nc as usize;
            let w = grid[nr][nc];
            let nd = d.saturating_add(w);
            let idx = nr * n + nc;
            if nd < dist[idx] {
                dist[idx] = nd;
                if w == 0 {
                    dq.push_front((nr, nc));
                } else {
                    dq.push_back((nr, nc));
                }
            }
        }
    }
    dist[m * n - 1]
}

fn main() {
    println!(
        "{}",
        minimum_obstacles(vec![vec![0, 1, 1], vec![0, 1, 0], vec![0, 0, 0]])
    );
}

#[cfg(test)]
mod tests {
    use super::minimum_obstacles;

    #[test]
    fn basic() {
        assert_eq!(
            minimum_obstacles(vec![vec![0, 1, 1], vec![0, 1, 0], vec![0, 0, 0]]),
            0
        );
    }

    #[test]
    fn must_remove() {
        assert_eq!(minimum_obstacles(vec![vec![0, 1], vec![1, 0]]), 1);
    }
}

