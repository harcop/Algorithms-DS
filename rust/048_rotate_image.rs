/// LeetCode #48 - Rotate Image
fn rotate(matrix: &mut [Vec<i32>]) {
    let n = matrix.len();

    for i in 0..n {
        for j in (i + 1)..n {
            let temp = matrix[i][j];
            matrix[i][j] = matrix[j][i];
            matrix[j][i] = temp;
        }
    }

    for row in matrix.iter_mut() {
        row.reverse();
    }
}

fn main() {
    let mut m = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
    rotate(&mut m);
    println!("{m:?}");
}

#[cfg(test)]
mod tests {
    use super::rotate;

    #[test]
    fn example_one() {
        let mut m = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
        rotate(&mut m);
        assert_eq!(m, vec![vec![7, 4, 1], vec![8, 5, 2], vec![9, 6, 3]]);
    }

    #[test]
    fn example_two() {
        let mut m = vec![
            vec![5, 1, 9, 11],
            vec![2, 4, 8, 10],
            vec![13, 3, 6, 7],
            vec![15, 14, 12, 16],
        ];
        rotate(&mut m);
        assert_eq!(
            m,
            vec![
                vec![15, 13, 2, 5],
                vec![14, 3, 4, 1],
                vec![12, 6, 8, 9],
                vec![16, 7, 10, 11]
            ]
        );
    }
}
