/// LeetCode #867 - Transpose Matrix
fn transpose(matrix: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let rows = matrix.len();
    let cols = matrix[0].len();
    let mut res = vec![vec![0; rows]; cols];
    for r in 0..rows {
        for c in 0..cols {
            res[c][r] = matrix[r][c];
        }
    }
    res
}

fn main() {
    println!("{:?}", transpose(vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]]));
}

#[cfg(test)]
mod tests {
    use super::transpose;

    #[test]
    fn example_one() {
        assert_eq!(
            transpose(vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]]),
            vec![vec![1, 4, 7], vec![2, 5, 8], vec![3, 6, 9]]
        );
    }
}
