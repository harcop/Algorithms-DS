/// LeetCode #2128 - Remove All Ones With Row and Column Flips
fn remove_ones(grid: Vec<Vec<i32>>) -> bool {
    let first = &grid[0];
    grid.iter().all(|row| {
        row.iter().zip(first.iter()).all(|(&a, &b)| a == b)
            || row.iter().zip(first.iter()).all(|(&a, &b)| a != b)
    })
}

fn main() {
    println!(
        "{}",
        remove_ones(vec![vec![0, 1, 0], vec![1, 0, 1], vec![0, 1, 0]])
    );
}

#[cfg(test)]
mod tests {
    use super::remove_ones;

    #[test]
    fn example_one() {
        assert!(remove_ones(vec![
            vec![0, 1, 0],
            vec![1, 0, 1],
            vec![0, 1, 0]
        ]));
    }

    #[test]
    fn example_two() {
        assert!(!remove_ones(vec![
            vec![1, 1, 0],
            vec![0, 0, 0],
            vec![0, 0, 0]
        ]));
    }

    #[test]
    fn example_three() {
        assert!(remove_ones(vec![vec![0]]));
    }
}
