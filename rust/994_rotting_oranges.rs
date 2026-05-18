/// LeetCode #994 - Rotting Oranges
use std::collections::VecDeque;

fn oranges_rotting(mut grid: Vec<Vec<i32>>) -> i32 {
    let rows = grid.len();
    let cols = grid[0].len();
    let mut q = VecDeque::new();
    let mut fresh = 0i32;
    for r in 0..rows {
        for c in 0..cols {
            match grid[r][c] {
                2 => q.push_back((r, c, 0i32)),
                1 => fresh += 1,
                _ => {}
            }
        }
    }
    let mut minutes = 0i32;
    while let Some((r, c, t)) = q.pop_front() {
        minutes = t;
        for (dr, dc) in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
            let nr = r as i32 + dr;
            let nc = c as i32 + dc;
            if nr < 0 || nc < 0 || nr as usize >= rows || nc as usize >= cols {
                continue;
            }
            let nr = nr as usize;
            let nc = nc as usize;
            if grid[nr][nc] == 1 {
                grid[nr][nc] = 2;
                fresh -= 1;
                q.push_back((nr, nc, t + 1));
            }
        }
    }
    if fresh == 0 { minutes } else { -1 }
}

fn main() {
    let mut grid = vec![vec![2, 1, 1], vec![1, 1, 0], vec![0, 1, 1]];
    println!("{}", oranges_rotting(grid));
}

#[cfg(test)]
mod tests {
    use super::oranges_rotting;

    #[test]
    fn example_one() {
        assert_eq!(
            oranges_rotting(vec![vec![2, 1, 1], vec![1, 1, 0], vec![0, 1, 1]]),
            4
        );
    }

    #[test]
    fn example_two() {
        assert_eq!(oranges_rotting(vec![vec![2, 1, 1], vec![0, 1, 1], vec![1, 0, 1]]), -1);
    }
}
