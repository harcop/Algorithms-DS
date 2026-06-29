/// LeetCode #2174 - Remove All Ones With Row and Column Flips II
fn remove_ones(grid: Vec<Vec<i32>>) -> i32 {
    fn dfs(grid: &mut [Vec<i32>]) -> i32 {
        let m = grid.len();
        let n = grid[0].len();

        for i in 0..m {
            for j in 0..n {
                if grid[i][j] == 1 {
                    let row = grid[i].clone();
                    for cell in &mut grid[i] {
                        *cell = 0;
                    }
                    let by_row = 1 + dfs(grid);
                    grid[i] = row;

                    let col: Vec<i32> = (0..m).map(|r| grid[r][j]).collect();
                    for r in 0..m {
                        grid[r][j] = 0;
                    }
                    let by_col = 1 + dfs(grid);
                    for r in 0..m {
                        grid[r][j] = col[r];
                    }

                    return by_row.min(by_col);
                }
            }
        }

        0
    }

    let mut grid = grid;
    dfs(&mut grid)
}

fn main() {
    println!("{}", remove_ones(vec![vec![1, 1, 1], vec![1, 1, 1]]));
}

#[cfg(test)]
mod tests {
    use super::remove_ones;

    #[test]
    fn example_one() {
        assert_eq!(remove_ones(vec![vec![1, 1, 1], vec![1, 1, 1]]), 2);
    }

    #[test]
    fn example_two() {
        assert_eq!(remove_ones(vec![vec![0, 1, 0], vec![1, 0, 1]]), 2);
    }
}
