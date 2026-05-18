/// LeetCode #980 - Unique Paths III
fn unique_paths_iii(grid: Vec<Vec<i32>>) -> i32 {
    let rows = grid.len();
    let cols = grid[0].len();
    let mut start = (0usize, 0usize);
    let mut empty = 0i32;
    for r in 0..rows {
        for c in 0..cols {
            match grid[r][c] {
                1 => start = (r, c),
                0 => empty += 1,
                _ => {}
            }
        }
    }
    fn dfs(grid: &mut [Vec<i32>], r: usize, c: usize, left: i32) -> i32 {
        if grid[r][c] == 2 {
            return if left == -1 { 1 } else { 0 };
        }
        let saved = grid[r][c];
        grid[r][c] = -1;
        let mut paths = 0i32;
        for (dr, dc) in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
            let nr = r as i32 + dr;
            let nc = c as i32 + dc;
            if nr < 0
                || nc < 0
                || nr as usize >= grid.len()
                || nc as usize >= grid[0].len()
            {
                continue;
            }
            let nr = nr as usize;
            let nc = nc as usize;
            if grid[nr][nc] < 0 {
                continue;
            }
            paths += dfs(grid, nr, nc, left - 1);
        }
        grid[r][c] = saved;
        paths
    }
    let mut g = grid;
    dfs(&mut g, start.0, start.1, empty)
}

fn main() {
    let grid = vec![
        vec![1, 0, 0, 0],
        vec![0, 0, 0, 0],
        vec![0, 0, 2, -1],
    ];
    println!("{}", unique_paths_iii(grid));
}

#[cfg(test)]
mod tests {
    use super::unique_paths_iii;

    #[test]
    fn example_one() {
        let grid = vec![
            vec![1, 0, 0, 0],
            vec![0, 0, 0, 0],
            vec![0, 0, 2, -1],
        ];
        assert_eq!(unique_paths_iii(grid), 2);
    }

    #[test]
    fn example_two() {
        let grid = vec![vec![1, 0, 0, 0], vec![0, 0, 0, 0], vec![0, 0, 0, 2]];
        assert_eq!(unique_paths_iii(grid), 4);
    }
}
