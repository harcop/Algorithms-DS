/// LeetCode #1162 - As Far from Land as Possible
use std::collections::VecDeque;

fn max_distance(mut grid: Vec<Vec<i32>>) -> i32 {
    let n = grid.len();
    let mut q = VecDeque::new();
    let mut land = 0usize;
    for i in 0..n {
        for j in 0..n {
            if grid[i][j] == 1 {
                land += 1;
                q.push_back((i, j, 0i32));
            }
        }
    }
    if land == 0 || land == n * n {
        return -1;
    }
    let mut ans = -1i32;
    let dirs = [(0, 1), (0, -1), (1, 0), (-1, 0)];
    while let Some((i, j, d)) = q.pop_front() {
        for (di, dj) in dirs {
            let ni = i as i32 + di;
            let nj = j as i32 + dj;
            if ni < 0 || nj < 0 || ni >= n as i32 || nj >= n as i32 {
                continue;
            }
            let ni = ni as usize;
            let nj = nj as usize;
            if grid[ni][nj] == 1 {
                continue;
            }
            ans = ans.max(d + 1);
            grid[ni][nj] = 1;
            q.push_back((ni, nj, d + 1));
        }
    }
    ans
}

fn main() {
    let grid = vec![vec![1, 0, 1], vec![1, 0, 0], vec![1, 0, 0]];
    println!("{}", max_distance(grid));
}

#[cfg(test)]
mod tests {
    use super::max_distance;

    #[test]
    fn example_one() {
        let grid = vec![vec![1, 0, 1], vec![1, 0, 0], vec![1, 0, 0]];
        assert_eq!(max_distance(grid), 2);
    }

    #[test]
    fn example_two() {
        let grid = vec![vec![1, 0, 0], vec![0, 0, 0], vec![0, 0, 0]];
        assert_eq!(max_distance(grid), 4);
    }
}
