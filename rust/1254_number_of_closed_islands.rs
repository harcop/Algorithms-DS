/// LeetCode #1254 - Number of Closed Islands
fn closed_island(grid: Vec<Vec<i32>>) -> i32 {
    let rows = grid.len();
    if rows == 0 {
        return 0;
    }
    let cols = grid[0].len();
    let mut g = grid;
    fn dfs(g: &mut [Vec<i32>], r: usize, c: usize) {
        if r >= g.len() || c >= g[0].len() || g[r][c] == 1 {
            return;
        }
        g[r][c] = 1;
        if r > 0 {
            dfs(g, r - 1, c);
        }
        if c > 0 {
            dfs(g, r, c - 1);
        }
        if r + 1 < g.len() {
            dfs(g, r + 1, c);
        }
        if c + 1 < g[0].len() {
            dfs(g, r, c + 1);
        }
    }
    for c in 0..cols {
        dfs(&mut g, 0, c);
        dfs(&mut g, rows - 1, c);
    }
    for r in 0..rows {
        dfs(&mut g, r, 0);
        dfs(&mut g, r, cols - 1);
    }
    let mut ans = 0i32;
    for r in 0..rows {
        for c in 0..cols {
            if g[r][c] == 0 {
                ans += 1;
                dfs(&mut g, r, c);
            }
        }
    }
    ans
}

fn main() {
    let grid = vec![
        vec![1, 1, 1, 1, 1, 1, 1, 0],
        vec![1, 0, 0, 0, 0, 1, 1, 0],
        vec![1, 0, 1, 0, 1, 1, 1, 0],
        vec![1, 0, 0, 0, 0, 1, 0, 1],
        vec![1, 1, 1, 1, 1, 1, 1, 0],
    ];
    println!("{}", closed_island(grid));
}

#[cfg(test)]
mod tests {
    use super::closed_island;

    #[test]
    fn example_one() {
        let grid = vec![
            vec![1, 1, 1, 1, 1, 1, 1, 0],
            vec![1, 0, 0, 0, 0, 1, 1, 0],
            vec![1, 0, 1, 0, 1, 1, 1, 0],
            vec![1, 0, 0, 0, 0, 1, 0, 1],
            vec![1, 1, 1, 1, 1, 1, 1, 0],
        ];
        assert_eq!(closed_island(grid), 2);
    }

    #[test]
    fn example_two() {
        assert_eq!(closed_island(vec![vec![0, 0, 1, 0, 0], vec![0, 1, 0, 1, 0], vec![0, 0, 1, 0, 0]]), 1);
    }
}
