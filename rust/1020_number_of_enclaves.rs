/// LeetCode #1020 - Number of Enclaves
fn num_enclaves(grid: Vec<Vec<i32>>) -> i32 {
    let rows = grid.len();
    let cols = grid[0].len();
    let mut g = grid;
    fn dfs(g: &mut [Vec<i32>], r: usize, c: usize) {
        if r >= g.len() || c >= g[0].len() || g[r][c] == 0 {
            return;
        }
        g[r][c] = 0;
        if r > 0 { dfs(g, r - 1, c); }
        if c > 0 { dfs(g, r, c - 1); }
        if r + 1 < g.len() { dfs(g, r + 1, c); }
        if c + 1 < g[0].len() { dfs(g, r, c + 1); }
    }
    for c in 0..cols {
        dfs(&mut g, 0, c);
        dfs(&mut g, rows - 1, c);
    }
    for r in 0..rows {
        dfs(&mut g, r, 0);
        dfs(&mut g, r, cols - 1);
    }
    g.iter().flatten().filter(|&&x| x == 1).count() as i32
}

fn main() {
    println!("{}", num_enclaves(vec![vec![0, 0, 0, 0], vec![1, 0, 0, 0], vec![0, 1, 1, 0], vec![0, 1, 1, 0]]));
}

#[cfg(test)]
mod tests {
    use super::num_enclaves;

    #[test]
    fn example_one() {
        assert_eq!(
            num_enclaves(vec![vec![0, 0, 0, 0], vec![1, 0, 0, 0], vec![0, 1, 1, 0], vec![0, 1, 1, 0]]),
            3
        );
    }
}
