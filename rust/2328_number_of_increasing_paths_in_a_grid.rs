/// LeetCode #2328 - Number of Increasing Paths in a Grid
fn count_paths(grid: Vec<Vec<i32>>) -> i32 {
    const MOD: i32 = 1_000_000_007;
    let m = grid.len();
    let n = grid[0].len();
    let mut f = vec![vec![0i32; n]; m];
    let dirs = [-1i32, 0, 1, 0, -1];

    fn dfs(
        i: usize,
        j: usize,
        grid: &[Vec<i32>],
        f: &mut [Vec<i32>],
        dirs: &[i32; 5],
    ) -> i32 {
        const MOD: i32 = 1_000_000_007;
        if f[i][j] != 0 {
            return f[i][j];
        }
        let mut ans = 1;
        for k in 0..4 {
            let x = i as i32 + dirs[k];
            let y = j as i32 + dirs[k + 1];
            if x >= 0
                && x < grid.len() as i32
                && y >= 0
                && y < grid[0].len() as i32
                && grid[i][j] < grid[x as usize][y as usize]
            {
                ans = (ans + dfs(x as usize, y as usize, grid, f, dirs)) % MOD;
            }
        }
        f[i][j] = ans;
        ans
    }

    let mut ans = 0i32;
    for i in 0..m {
        for j in 0..n {
            ans = (ans + dfs(i, j, &grid, &mut f, &dirs)) % MOD;
        }
    }
    ans
}

fn main() {
    println!("{}", count_paths(vec![vec![1, 1], vec![3, 4]]));
}

#[cfg(test)]
mod tests {
    use super::count_paths;

    #[test]
    fn example_one() {
        assert_eq!(count_paths(vec![vec![1, 1], vec![3, 4]]), 8);
    }

    #[test]
    fn example_two() {
        assert_eq!(count_paths(vec![vec![1], vec![2]]), 3);
    }
}
