/// LeetCode #1914 - Cyclically Rotating a Grid
fn rotate_grid(mut grid: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
    let m = grid.len();
    let n = grid[0].len();
    let layers = m.min(n) / 2;

    for p in 0..layers {
        let mut nums = Vec::new();
        for j in p..n - p - 1 {
            nums.push(grid[p][j]);
        }
        for i in p..m - p - 1 {
            nums.push(grid[i][n - p - 1]);
        }
        for j in (p + 1..n - p).rev() {
            nums.push(grid[m - p - 1][j]);
        }
        for i in (p + 1..m - p).rev() {
            nums.push(grid[i][p]);
        }
        let len = nums.len();
        if len == 0 {
            continue;
        }
        let shift = (k as usize) % len;
        if shift == 0 {
            continue;
        }
        let rotated: Vec<i32> = nums[shift..].iter().chain(nums[..shift].iter()).copied().collect();
        let mut idx = 0usize;
        for j in p..n - p - 1 {
            grid[p][j] = rotated[idx];
            idx += 1;
        }
        for i in p..m - p - 1 {
            grid[i][n - p - 1] = rotated[idx];
            idx += 1;
        }
        for j in (p + 1..n - p).rev() {
            grid[m - p - 1][j] = rotated[idx];
            idx += 1;
        }
        for i in (p + 1..m - p).rev() {
            grid[i][p] = rotated[idx];
            idx += 1;
        }
    }
    grid
}

fn main() {
    println!("{:?}", rotate_grid(vec![vec![40, 10], vec![30, 20]], 1));
}

#[cfg(test)]
mod tests {
    use super::rotate_grid;

    #[test]
    fn example_one() {
        assert_eq!(
            rotate_grid(vec![vec![40, 10], vec![30, 20]], 1),
            vec![vec![10, 20], vec![40, 30]]
        );
    }
}
