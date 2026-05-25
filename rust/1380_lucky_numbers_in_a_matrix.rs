/// LeetCode #1380 - Lucky Numbers In A Matrix

fn lucky_numbers(matrix: Vec<Vec<i32>>) -> Vec<i32> {
    let m = matrix.len();
    let n = matrix[0].len();
    let mut row_min = vec![i32::MAX; m];
    let mut col_max = vec![i32::MIN; n];
    for i in 0..m {
        for j in 0..n {
            row_min[i] = row_min[i].min(matrix[i][j]);
            col_max[j] = col_max[j].max(matrix[i][j]);
        }
    }
    let mut out = vec![];
    for i in 0..m {
        for j in 0..n {
            if matrix[i][j] == row_min[i] && matrix[i][j] == col_max[j] {
                out.push(matrix[i][j]);
            }
        }
    }
    out
}

fn main() {
    println!("{:?}", lucky_numbers(vec![vec![3, 7, 8], vec![9, 11, 13], vec![15, 16, 17]]));
}

#[cfg(test)]
mod tests {
    use super::lucky_numbers;

    #[test]
    fn example_one() {
        assert_eq!(lucky_numbers(vec![vec![3, 7, 8], vec![9, 11, 13], vec![15, 16, 17]]), vec![15]);
    }

    #[test]
    fn example_two() {
        assert_eq!(lucky_numbers(vec![vec![7, 8], vec![1, 2]]), vec![7]);
    }
}
