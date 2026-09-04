/// LeetCode #489 - Robot Room Cleaner
use std::collections::HashSet;

fn clean_room(grid: Vec<Vec<i32>>, row: i32, col: i32) -> i32 {
    let m = grid.len() as i32;
    let n = grid[0].len() as i32;
    let mut cleaned = HashSet::new();
    let dirs = [(-1, 0), (0, 1), (1, 0), (0, -1)];
    dfs(row, col, 0, &grid, m, n, &mut cleaned, &dirs);
    cleaned.len() as i32
}

fn dfs(
    r: i32,
    c: i32,
    dir: usize,
    grid: &[Vec<i32>],
    m: i32,
    n: i32,
    cleaned: &mut HashSet<(i32, i32)>,
    dirs: &[(i32, i32); 4],
) {
    cleaned.insert((r, c));
    for k in 0..4 {
        let nd = (dir + k) % 4;
        let nr = r + dirs[nd].0;
        let nc = c + dirs[nd].1;
        if nr >= 0
            && nc >= 0
            && nr < m
            && nc < n
            && grid[nr as usize][nc as usize] == 1
            && !cleaned.contains(&(nr, nc))
        {
            dfs(nr, nc, nd, grid, m, n, cleaned, dirs);
        }
    }
}

fn main() {
    let grid = vec![
        vec![1, 1, 1, 1, 1, 0, 1, 1],
        vec![1, 1, 1, 1, 1, 0, 1, 1],
        vec![1, 0, 1, 1, 1, 1, 1, 1],
        vec![0, 0, 0, 1, 0, 0, 0, 0],
        vec![1, 1, 1, 1, 1, 1, 1, 1],
    ];
    println!("{}", clean_room(grid, 1, 3));
}

#[cfg(test)]
mod tests {
    use super::clean_room;

    #[test]
    fn example_one() {
        let grid = vec![
            vec![1, 1, 1, 1, 1, 0, 1, 1],
            vec![1, 1, 1, 1, 1, 0, 1, 1],
            vec![1, 0, 1, 1, 1, 1, 1, 1],
            vec![0, 0, 0, 1, 0, 0, 0, 0],
            vec![1, 1, 1, 1, 1, 1, 1, 1],
        ];
        assert_eq!(clean_room(grid, 1, 3), 30);
    }

    #[test]
    fn example_two() {
        let grid = vec![vec![1]];
        assert_eq!(clean_room(grid, 0, 0), 1);
    }
}
