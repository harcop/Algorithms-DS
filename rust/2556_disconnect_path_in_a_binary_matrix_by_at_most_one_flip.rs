/// LeetCode #2556 - Disconnect Path in a Binary Matrix by at Most One Flip
fn dfs(grid: &mut Vec<Vec<i32>>, i: usize, j: usize) -> bool {
    let m = grid.len();
    let n = grid[0].len();
    if i >= m || j >= n || grid[i][j] == 0 {
        return false;
    }
    if i == m - 1 && j == n - 1 {
        return true;
    }
    grid[i][j] = 0;
    dfs(grid, i + 1, j) || dfs(grid, i, j + 1)
}

fn is_possible_to_cut_path(mut grid: Vec<Vec<i32>>) -> bool {
    let m = grid.len();
    let n = grid[0].len();
    let a = dfs(&mut grid, 0, 0);
    grid[0][0] = 1;
    grid[m - 1][n - 1] = 1;
    let b = dfs(&mut grid, 0, 0);
    !(a && b)
}

fn main() {
    let grid = vec![vec![1, 1, 1], vec![1, 0, 0], vec![1, 1, 1]];
    println!("{}", is_possible_to_cut_path(grid));
}

#[cfg(test)]
mod tests {
    use super::is_possible_to_cut_path;

    #[test]
    fn example_one() {
        let grid = vec![vec![1, 1, 1], vec![1, 0, 0], vec![1, 1, 1]];
        assert!(is_possible_to_cut_path(grid));
    }

    #[test]
    fn example_two() {
        let grid = vec![vec![1, 1, 1], vec![1, 0, 1], vec![1, 1, 1]];
        assert!(!is_possible_to_cut_path(grid));
    }
}
