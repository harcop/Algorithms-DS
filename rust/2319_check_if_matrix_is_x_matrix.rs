/// LeetCode #2319 - Check if Matrix Is X-Matrix
fn check_x_matrix(grid: Vec<Vec<i32>>) -> bool {
    let n = grid.len();
    for i in 0..n {
        for j in 0..n {
            if i == j || i + j == n - 1 {
                if grid[i][j] == 0 {
                    return false;
                }
            } else if grid[i][j] != 0 {
                return false;
            }
        }
    }
    true
}

fn main() {
    println!(
        "{}",
        check_x_matrix(vec![
            vec![2, 0, 0, 1],
            vec![0, 3, 1, 0],
            vec![0, 1, 3, 0],
            vec![1, 0, 0, 2]
        ])
    );
}

#[cfg(test)]
mod tests {
    use super::check_x_matrix;

    #[test]
    fn example_one() {
        assert!(check_x_matrix(vec![
            vec![2, 0, 0, 1],
            vec![0, 3, 1, 0],
            vec![0, 1, 3, 0],
            vec![1, 0, 0, 2]
        ]));
    }

    #[test]
    fn example_two() {
        assert!(!check_x_matrix(vec![vec![5, 7, 0], vec![0, 3, 1], vec![0, 5, 0]]));
    }
}
