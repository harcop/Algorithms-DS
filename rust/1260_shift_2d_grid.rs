/// LeetCode #1260 - Shift 2D Grid
fn shift_grid(grid: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
    let m = grid.len();
    let n = grid[0].len();
    let total = (m * n) as i32;
    let k = (k % total) as usize;
    let flat: Vec<i32> = grid.iter().flatten().copied().collect();
    let mut rotated = vec![0i32; total as usize];
    for i in 0..total as usize {
        rotated[(i + k) % total as usize] = flat[i];
    }
    let mut out = vec![vec![0; n]; m];
    for r in 0..m {
        for c in 0..n {
            out[r][c] = rotated[r * n + c];
        }
    }
    out
}

fn main() {
    println!(
        "{:?}",
        shift_grid(vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]], 1)
    );
}

#[cfg(test)]
mod tests {
    use super::shift_grid;

    #[test]
    fn example_one() {
        assert_eq!(
            shift_grid(vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]], 1),
            vec![vec![9, 1, 2], vec![3, 4, 5], vec![6, 7, 8]]
        );
    }

    #[test]
    fn example_two() {
        assert_eq!(
            shift_grid(vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]], 9),
            vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]]
        );
    }
}
