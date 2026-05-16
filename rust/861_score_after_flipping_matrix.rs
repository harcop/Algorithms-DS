/// LeetCode #861 - Score After Flipping Matrix
fn matrix_score(grid: Vec<Vec<i32>>) -> i32 {
    let rows = grid.len();
    let cols = grid[0].len();
    let mut ans = rows as i32 * (1 << (cols - 1));
    for c in 1..cols {
        let mut ones = 0;
        for r in 0..rows {
            if grid[r][0] == grid[r][c] {
                ones += 1;
            }
        }
        ans += (ones.max(rows - ones) as i32) * (1 << (cols - 1 - c));
    }
    ans
}

fn main() {
    println!("{}", matrix_score(vec![vec![0, 0, 1, 1], vec![1, 0, 1, 0], vec![1, 1, 0, 0]]));
}

#[cfg(test)]
mod tests {
    use super::matrix_score;

    #[test]
    fn example_one() {
        assert_eq!(
            matrix_score(vec![vec![0, 0, 1, 1], vec![1, 0, 1, 0], vec![1, 1, 0, 0]]),
            39
        );
    }
}
