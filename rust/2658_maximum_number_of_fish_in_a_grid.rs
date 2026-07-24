/// LeetCode #2658 - Maximum Number of Fish in a Grid
fn find_max_fish(mut grid: Vec<Vec<i32>>) -> i32 {
    let m = grid.len();
    let n = grid[0].len();
    let dirs = [-1, 0, 1, 0, -1];

    fn dfs(grid: &mut [Vec<i32>], i: usize, j: usize, m: usize, n: usize, dirs: &[i32; 5]) -> i32 {
        let mut cnt = grid[i][j];
        grid[i][j] = 0;
        for k in 0..4 {
            let x = i as i32 + dirs[k];
            let y = j as i32 + dirs[k + 1];
            if x >= 0 && (x as usize) < m && y >= 0 && (y as usize) < n && grid[x as usize][y as usize] > 0
            {
                cnt += dfs(grid, x as usize, y as usize, m, n, dirs);
            }
        }
        cnt
    }

    let mut ans = 0;
    for i in 0..m {
        for j in 0..n {
            if grid[i][j] > 0 {
                ans = ans.max(dfs(&mut grid, i, j, m, n, &dirs));
            }
        }
    }
    ans
}

fn main() {
    println!(
        "{}",
        find_max_fish(vec![
            vec![0, 2, 1, 0],
            vec![4, 0, 0, 3],
            vec![1, 0, 0, 4],
            vec![0, 3, 2, 0]
        ])
    );
}

#[cfg(test)]
mod tests {
    use super::find_max_fish;

    #[test]
    fn example_one() {
        assert_eq!(
            find_max_fish(vec![
                vec![0, 2, 1, 0],
                vec![4, 0, 0, 3],
                vec![1, 0, 0, 4],
                vec![0, 3, 2, 0]
            ]),
            7
        );
    }

    #[test]
    fn example_two() {
        assert_eq!(
            find_max_fish(vec![
                vec![1, 0, 0, 0],
                vec![0, 0, 0, 0],
                vec![0, 0, 0, 0],
                vec![0, 0, 0, 1]
            ]),
            1
        );
    }
}
