/// LeetCode #1368 - Minimum Cost To Make At Least One Valid Path In A Grid

use std::collections::VecDeque;

fn min_cost(grid: Vec<Vec<i32>>) -> i32 {
    let m = grid.len();
    let n = grid[0].len();
    let dirs = [(0, 1), (0, -1), (1, 0), (-1, 0)];
    let mut dist = vec![vec![i32::MAX; n]; m];
    dist[0][0] = 0;
    let mut dq = VecDeque::from([(0usize, 0usize)]);
    while let Some((r, c)) = dq.pop_front() {
        let d = dist[r][c];
        let arrow = grid[r][c] as usize - 1;
        for (i, (dr, dc)) in dirs.iter().enumerate() {
            let nr = r as i32 + dr;
            let nc = c as i32 + dc;
            if nr < 0 || nc < 0 || nr >= m as i32 || nc >= n as i32 {
                continue;
            }
            let nr = nr as usize;
            let nc = nc as usize;
            let cost = d + if i == arrow { 0 } else { 1 };
            if cost < dist[nr][nc] {
                dist[nr][nc] = cost;
                if i == arrow {
                    dq.push_front((nr, nc));
                } else {
                    dq.push_back((nr, nc));
                }
            }
        }
    }
    dist[m - 1][n - 1]
}

fn main() {
    println!("{}", min_cost(vec![vec![1, 1, 1, 1], vec![2, 2, 2, 2], vec![1, 1, 1, 1], vec![2, 2, 2, 2]]));
}

#[cfg(test)]
mod tests {
    use super::min_cost;

    #[test]
    fn example_one() {
        assert_eq!(min_cost(vec![vec![1, 1, 1, 1], vec![2, 2, 2, 2], vec![1, 1, 1, 1], vec![2, 2, 2, 2]]), 3);
    }

    #[test]
    fn example_two() {
        assert_eq!(min_cost(vec![vec![1, 1, 3], vec![3, 2, 2], vec![1, 1, 4]]), 0);
    }
}
