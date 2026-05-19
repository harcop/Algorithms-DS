/// LeetCode #1091 - Shortest Path in Binary Matrix
use std::collections::VecDeque;

fn shortest_path_binary_matrix(grid: Vec<Vec<i32>>) -> i32 {
    let n = grid.len();
    if grid[0][0] == 1 || grid[n - 1][n - 1] == 1 {
        return -1;
    }
    if n == 1 {
        return 1;
    }
    let mut g = grid;
    g[0][0] = 1;
    let mut q = VecDeque::from([(0usize, 0usize)]);
    let dirs = [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];
    let mut dist = 1i32;
    while !q.is_empty() {
        dist += 1;
        for _ in 0..q.len() {
            let (r, c) = q.pop_front().unwrap();
            for (dr, dc) in dirs {
                let nr = r as i32 + dr;
                let nc = c as i32 + dc;
                if nr < 0 || nc < 0 || nr >= n as i32 || nc >= n as i32 {
                    continue;
                }
                let nr = nr as usize;
                let nc = nc as usize;
                if g[nr][nc] == 1 {
                    continue;
                }
                if nr == n - 1 && nc == n - 1 {
                    return dist;
                }
                g[nr][nc] = 1;
                q.push_back((nr, nc));
            }
        }
    }
    -1
}

fn main() {
    let g = vec![vec![0, 1], vec![1, 0]];
    println!("{}", shortest_path_binary_matrix(g));
}

#[cfg(test)]
mod tests {
    use super::shortest_path_binary_matrix;

    #[test]
    fn example_one() {
        assert_eq!(
            shortest_path_binary_matrix(vec![vec![0, 1], vec![1, 0]]),
            2
        );
    }

    #[test]
    fn example_two() {
        assert_eq!(
            shortest_path_binary_matrix(vec![
                vec![0, 0, 0],
                vec![1, 1, 0],
                vec![1, 1, 0]
            ]),
            4
        );
    }
}
