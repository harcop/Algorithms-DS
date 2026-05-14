/// LeetCode #741 - Cherry Pickup
use std::collections::HashMap;

fn cherry_pickup(grid: Vec<Vec<i32>>) -> i32 {
    let mut memo: HashMap<(usize, usize, usize), i32> = HashMap::new();

    fn dfs(
        r1: usize,
        c1: usize,
        r2: usize,
        grid: &Vec<Vec<i32>>,
        memo: &mut HashMap<(usize, usize, usize), i32>,
    ) -> i32 {
        let c2 = r1 + c1 - r2;
        let n = grid.len();
        if r1 >= n || c1 >= n || r2 >= n || c2 >= n {
            return i32::MIN / 4;
        }
        if grid[r1][c1] == -1 || grid[r2][c2] == -1 {
            return i32::MIN / 4;
        }
        if r1 == n - 1 && c1 == n - 1 {
            return grid[n - 1][n - 1];
        }
        if let Some(&v) = memo.get(&(r1, c1, r2)) {
            return v;
        }
        let best = dfs(r1 + 1, c1, r2 + 1, grid, memo)
            .max(dfs(r1 + 1, c1, r2, grid, memo))
            .max(dfs(r1, c1 + 1, r2 + 1, grid, memo))
            .max(dfs(r1, c1 + 1, r2, grid, memo));
        if best < -1_000_000_000 {
            memo.insert((r1, c1, r2), i32::MIN / 4);
            return i32::MIN / 4;
        }
        let mut gain = grid[r1][c1];
        if r1 != r2 {
            gain += grid[r2][c2];
        }
        let ans = best + gain;
        memo.insert((r1, c1, r2), ans);
        ans
    }

    let a = dfs(0, 0, 0, &grid, &mut memo);
    if a < 0 {
        0
    } else {
        a
    }
}

fn main() {
    let g = vec![vec![0, 1, -1], vec![1, 0, -1], vec![1, 1, 1]];
    println!("{}", cherry_pickup(g));
}

#[cfg(test)]
mod tests {
    use super::cherry_pickup;

    #[test]
    fn example_one() {
        let g = vec![vec![0, 1, -1], vec![1, 0, -1], vec![1, 1, 1]];
        assert_eq!(cherry_pickup(g), 5);
    }
}
