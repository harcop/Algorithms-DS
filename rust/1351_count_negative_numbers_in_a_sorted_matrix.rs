/// LeetCode #1351 - Count Negative Numbers In A Sorted Matrix

fn count_negatives(grid: Vec<Vec<i32>>) -> i32 {
    let mut count = 0i32;
    let mut row = 0usize;
    let mut col = grid[0].len().wrapping_sub(1);
    while row < grid.len() && col < grid[0].len() {
        if grid[row][col] < 0 {
            count += (grid.len() - row) as i32;
            col = col.wrapping_sub(1);
        } else {
            row += 1;
        }
    }
    count
}

fn main() {
    println!("{}", count_negatives(vec![vec![4, 3, 2, -1], vec![3, 2, 1, -1], vec![1, 1, -1, -2], vec![-1, -1, -2, -3]]));
}

#[cfg(test)]
mod tests {
    use super::count_negatives;

    #[test]
    fn example_one() {
        assert_eq!(count_negatives(vec![vec![4, 3, 2, -1], vec![3, 2, 1, -1], vec![1, 1, -1, -2], vec![-1, -1, -2, -3]]), 8);
    }

    #[test]
    fn example_two() {
        assert_eq!(count_negatives(vec![vec![3, 2], vec![1, 0]]), 0);
    }
}
