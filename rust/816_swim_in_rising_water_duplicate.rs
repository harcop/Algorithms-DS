/// LeetCode #778 - Swim in Rising Water (duplicate slot of #816)
use std::collections::VecDeque;

fn swim_in_water(grid: Vec<Vec<i32>>) -> i32 {
    let n = grid.len();
    let mut lo = grid[0][0];
    let mut hi = 0;
    for row in &grid {
        for &v in row {
            hi = hi.max(v);
        }
    }
    while lo < hi {
        let mid = (lo + hi) / 2;
        if can_reach(&grid, mid) {
            hi = mid;
        } else {
            lo = mid + 1;
        }
    }
    lo
}

fn can_reach(grid: &[Vec<i32>], t: i32) -> bool {
    let n = grid.len();
    if grid[0][0] > t {
        return false;
    }
    let mut seen = vec![vec![false; n]; n];
    let mut q = VecDeque::new();
    q.push_back((0usize, 0usize));
    seen[0][0] = true;
    let dirs = [(0isize, 1isize), (0, -1), (1, 0), (-1, 0)];
    while let Some((r, c)) = q.pop_front() {
        if r == n - 1 && c == n - 1 {
            return true;
        }
        for (dr, dc) in dirs {
            let nr = r as isize + dr;
            let nc = c as isize + dc;
            if nr < 0 || nc < 0 || nr >= n as isize || nc >= n as isize {
                continue;
            }
            let nr = nr as usize;
            let nc = nc as usize;
            if !seen[nr][nc] && grid[nr][nc] <= t {
                seen[nr][nc] = true;
                q.push_back((nr, nc));
            }
        }
    }
    false
}

fn main() {
    println!("{}", swim_in_water(vec![vec![0, 2], vec![1, 3]]));
}

#[cfg(test)]
mod tests {
    use super::swim_in_water;

    #[test]
    fn example_one() {
        assert_eq!(swim_in_water(vec![vec![0, 2], vec![1, 3]]), 3);
    }

    #[test]
    fn example_two() {
        let grid = vec![
            vec![0, 1, 2, 3, 4],
            vec![24, 23, 22, 21, 5],
            vec![12, 13, 14, 15, 16],
            vec![11, 17, 18, 19, 20],
            vec![10, 9, 8, 7, 6],
        ];
        assert_eq!(swim_in_water(grid), 16);
    }
}
