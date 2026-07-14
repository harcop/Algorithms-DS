/// LeetCode #2387 - Median of a Row Wise Sorted Matrix
fn matrix_median(grid: Vec<Vec<i32>>) -> i32 {
    let m = grid.len();
    let n = grid[0].len();
    let target = (m * n + 1) / 2;

    let count = |x: i32| -> usize {
        grid.iter()
            .map(|row| row.partition_point(|&v| v <= x))
            .sum()
    };

    let mut left = 0;
    let mut right = 1_000_010;
    while left < right {
        let mid = (left + right) / 2;
        if count(mid) >= target {
            right = mid;
        } else {
            left = mid + 1;
        }
    }
    left
}

fn main() {
    println!(
        "{}",
        matrix_median(vec![vec![1, 1, 2], vec![2, 3, 3], vec![1, 3, 4]])
    );
}

#[cfg(test)]
mod tests {
    use super::matrix_median;

    #[test]
    fn example_one() {
        assert_eq!(
            matrix_median(vec![vec![1, 1, 2], vec![2, 3, 3], vec![1, 3, 4]]),
            2
        );
    }

    #[test]
    fn example_two() {
        assert_eq!(matrix_median(vec![vec![1, 1, 3, 3, 4]]), 3);
    }
}
