/// LeetCode #3142 - Check if Grid Satisfies Conditions
fn satisfies_conditions(grid: Vec<Vec<i32>>) -> bool {
    let m = grid.len();
    let n = grid[0].len();
    for i in 0..m {
        for j in 0..n {
            let x = grid[i][j];
            if i + 1 < m && x != grid[i + 1][j] {
                return false;
            }
            if j + 1 < n && x == grid[i][j + 1] {
                return false;
            }
        }
    }
    true
}

fn main() {
    println!(
        "{}",
        satisfies_conditions(vec![vec![1, 0, 2], vec![1, 0, 2]])
    );
}

#[cfg(test)]
mod tests {
    use super::satisfies_conditions;

    #[test]
    fn example1() {
        assert!(satisfies_conditions(vec![vec![1, 0, 2], vec![1, 0, 2]]));
    }

    #[test]
    fn example2() {
        assert!(!satisfies_conditions(vec![vec![1, 1, 1], vec![0, 0, 0]]));
    }

    #[test]
    fn example3() {
        assert!(!satisfies_conditions(vec![vec![1], vec![2], vec![3]]));
    }
}
