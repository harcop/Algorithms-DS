/// LeetCode #2482 - Difference Between Ones and Zeros in Row and Column
fn ones_minus_zeros(grid: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let m = grid.len();
    let n = grid[0].len();
    let mut ones_row = vec![0i32; m];
    let mut ones_col = vec![0i32; n];

    for (i, row) in grid.iter().enumerate() {
        for (j, &value) in row.iter().enumerate() {
            ones_row[i] += value;
            ones_col[j] += value;
        }
    }

    let mut answer = vec![vec![0; n]; m];
    for i in 0..m {
        for j in 0..n {
            answer[i][j] =
                ones_row[i] + ones_col[j] - (n as i32 - ones_row[i]) - (m as i32 - ones_col[j]);
        }
    }
    answer
}

fn main() {
    println!(
        "{:?}",
        ones_minus_zeros(vec![vec![0, 1, 1], vec![1, 0, 1], vec![0, 0, 1]])
    );
}

#[cfg(test)]
mod tests {
    use super::ones_minus_zeros;

    #[test]
    fn example_one() {
        assert_eq!(
            ones_minus_zeros(vec![vec![0, 1, 1], vec![1, 0, 1], vec![0, 0, 1]]),
            vec![vec![0, 0, 4], vec![0, 0, 4], vec![-2, -2, 2]]
        );
    }

    #[test]
    fn example_two() {
        assert_eq!(
            ones_minus_zeros(vec![vec![1, 1, 1], vec![1, 1, 1]]),
            vec![vec![5, 5, 5], vec![5, 5, 5]]
        );
    }
}
