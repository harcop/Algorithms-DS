/// LeetCode #1102 - Path With Maximum Minimum Value
use std::collections::BinaryHeap;

fn maximum_minimum_path(grid: Vec<Vec<i32>>) -> i32 {
    let n = grid.len();
    let m = grid[0].len();
    let mut heap = BinaryHeap::new();
    let mut seen = vec![vec![false; m]; n];
    heap.push((grid[0][0], 0usize, 0usize));
    let dirs = [(0, 1), (0, -1), (1, 0), (-1, 0)];
    while let Some((v, r, c)) = heap.pop() {
        if seen[r][c] {
            continue;
        }
        seen[r][c] = true;
        if r == n - 1 && c == m - 1 {
            return v;
        }
        for (dr, dc) in dirs {
            let nr = r as i32 + dr;
            let nc = c as i32 + dc;
            if nr < 0 || nc < 0 || nr >= n as i32 || nc >= m as i32 {
                continue;
            }
            let nr = nr as usize;
            let nc = nc as usize;
            if !seen[nr][nc] {
                heap.push((grid[nr][nc].min(v), nr, nc));
            }
        }
    }
    -1
}

fn main() {
    let g = vec![vec![5, 4, 5], vec![1, 2, 6], vec![7, 4, 6]];
    println!("{}", maximum_minimum_path(g));
}

#[cfg(test)]
mod tests {
    use super::maximum_minimum_path;

    #[test]
    fn example_one() {
        assert_eq!(
            maximum_minimum_path(vec![vec![5, 4, 5], vec![1, 2, 6], vec![7, 4, 6]]),
            4
        );
    }

    #[test]
    fn example_two() {
        assert_eq!(maximum_minimum_path(vec![vec![2, 2, 1, 2, 2, 2], vec![1, 2, 2, 2, 1, 2]]), 2);
    }
}
