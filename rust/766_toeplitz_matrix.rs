/// LeetCode #766 - Toeplitz Matrix
fn is_toeplitz_matrix(matrix: Vec<Vec<i32>>) -> bool {
    let m = matrix.len();
    let n = matrix[0].len();
    for i in 0..m {
        for j in 0..n {
            if i > 0 && j > 0 && matrix[i][j] != matrix[i - 1][j - 1] {
                return false;
            }
        }
    }
    true
}

fn main() {
    let g = vec![vec![1, 2, 3, 4], vec![5, 1, 2, 3], vec![9, 5, 1, 2]];
    println!("{}", is_toeplitz_matrix(g));
}

#[cfg(test)]
mod tests {
    use super::is_toeplitz_matrix;

    #[test]
    fn example_one() {
        let g = vec![vec![1, 2, 3, 4], vec![5, 1, 2, 3], vec![9, 5, 1, 2]];
        assert!(is_toeplitz_matrix(g));
    }
}
