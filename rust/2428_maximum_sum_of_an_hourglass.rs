/// LeetCode #2428 - Maximum Sum of an Hourglass
fn max_sum(grid: Vec<Vec<i32>>) -> i32 {
    let mut answer = i32::MIN;

    for row in 0..grid.len() - 2 {
        for col in 0..grid[0].len() - 2 {
            let sum = grid[row][col]
                + grid[row][col + 1]
                + grid[row][col + 2]
                + grid[row + 1][col + 1]
                + grid[row + 2][col]
                + grid[row + 2][col + 1]
                + grid[row + 2][col + 2];
            answer = answer.max(sum);
        }
    }

    answer
}

fn main() {
    println!(
        "{}",
        max_sum(vec![
            vec![6, 2, 1, 3],
            vec![4, 2, 1, 5],
            vec![9, 2, 8, 7],
            vec![4, 1, 2, 9]
        ])
    );
}

#[cfg(test)]
mod tests {
    use super::max_sum;

    #[test]
    fn example_one() {
        assert_eq!(
            max_sum(vec![
                vec![6, 2, 1, 3],
                vec![4, 2, 1, 5],
                vec![9, 2, 8, 7],
                vec![4, 1, 2, 9]
            ]),
            30
        );
    }

    #[test]
    fn single_hourglass() {
        assert_eq!(
            max_sum(vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]]),
            35
        );
    }
}
