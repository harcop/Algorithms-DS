/// LeetCode #2510 - Check if There is a Path With Equal Number of 0's And 1's
use std::collections::HashMap;

fn is_there_a_path(grid: Vec<Vec<i32>>) -> bool {
    let m = grid.len();
    let n = grid[0].len();
    let mut s = m + n - 1;
    if s % 2 == 1 {
        return false;
    }
    s >>= 1;
    let mut memo = HashMap::new();
    dfs(0, 0, 0, s, &grid, &mut memo)
}

fn dfs(
    i: usize,
    j: usize,
    mut k: i32,
    s: usize,
    grid: &[Vec<i32>],
    memo: &mut HashMap<(usize, usize, i32), bool>,
) -> bool {
    let m = grid.len();
    let n = grid[0].len();
    if i >= m || j >= n {
        return false;
    }
    k += grid[i][j];
    if let Some(&v) = memo.get(&(i, j, k)) {
        return v;
    }
    if k as usize > s || i + j + 1 - k as usize > s {
        return false;
    }
    if i == m - 1 && j == n - 1 {
        return k as usize == s;
    }
    let res = dfs(i + 1, j, k, s, grid, memo) || dfs(i, j + 1, k, s, grid, memo);
    memo.insert((i, j, k), res);
    res
}

fn main() {
    println!(
        "{}",
        is_there_a_path(vec![
            vec![0, 1, 0, 0],
            vec![0, 1, 0, 0],
            vec![1, 0, 1, 0]
        ])
    );
}

#[cfg(test)]
mod tests {
    use super::is_there_a_path;

    #[test]
    fn example_one() {
        assert!(is_there_a_path(vec![
            vec![0, 1, 0, 0],
            vec![0, 1, 0, 0],
            vec![1, 0, 1, 0]
        ]));
    }

    #[test]
    fn example_two() {
        assert!(!is_there_a_path(vec![
            vec![1, 1, 0],
            vec![0, 0, 1],
            vec![1, 0, 0]
        ]));
    }
}
