/// LeetCode #2267 - Check if There Is a Valid Parentheses String Path
fn has_valid_path(grid: Vec<Vec<char>>) -> bool {
    let m = grid.len();
    let n = grid[0].len();
    let max_k = m + n;
    let mut mem = vec![vec![vec![-1i8; max_k + 1]; n]; m];
    has_valid_path_dfs(&grid, 0, 0, 0, &mut mem)
}

fn has_valid_path_dfs(
    grid: &[Vec<char>],
    i: usize,
    j: usize,
    k: i32,
    mem: &mut [Vec<Vec<i8>>],
) -> bool {
    let m = grid.len();
    let n = grid[0].len();
    if i == m || j == n {
        return false;
    }

    let k = if grid[i][j] == '(' { k + 1 } else { k - 1 };
    if k < 0 {
        return false;
    }

    if i == m - 1 && j == n - 1 {
        return k == 0;
    }

    let ki = k as usize;
    if mem[i][j][ki] != -1 {
        return mem[i][j][ki] == 1;
    }

    let ans = has_valid_path_dfs(grid, i + 1, j, k, mem)
        || has_valid_path_dfs(grid, i, j + 1, k, mem);
    mem[i][j][ki] = if ans { 1 } else { 0 };
    ans
}

fn main() {
    println!(
        "{}",
        has_valid_path(vec![
            vec!['(', '(', '('],
            vec![')', '(', ')'],
            vec!['(', '(', ')'],
            vec!['(', '(', ')']
        ])
    );
}

#[cfg(test)]
mod tests {
    use super::has_valid_path;

    #[test]
    fn example_one() {
        assert!(has_valid_path(vec![
            vec!['(', '(', '('],
            vec![')', '(', ')'],
            vec!['(', '(', ')'],
            vec!['(', '(', ')']
        ]));
    }

    #[test]
    fn example_two() {
        assert!(!has_valid_path(vec![vec![')', ')'], vec!['(', '(']]));
    }
}
