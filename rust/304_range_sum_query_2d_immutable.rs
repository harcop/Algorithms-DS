/// LeetCode #304 - Range Sum Query 2D - Immutable
pub struct NumMatrix {
    pre: Vec<Vec<i32>>,
}

impl NumMatrix {
    fn new(matrix: Vec<Vec<i32>>) -> Self {
        let m = matrix.len();
        let n = if m == 0 { 0 } else { matrix[0].len() };
        let mut pre = vec![vec![0; n + 1]; m + 1];
        for i in 0..m {
            for j in 0..n {
                pre[i + 1][j + 1] =
                    matrix[i][j] + pre[i][j + 1] + pre[i + 1][j] - pre[i][j];
            }
        }
        NumMatrix { pre }
    }

    fn sum_region(&self, row1: i32, col1: i32, row2: i32, col2: i32) -> i32 {
        let r1 = row1 as usize;
        let c1 = col1 as usize;
        let r2 = row2 as usize + 1;
        let c2 = col2 as usize + 1;
        self.pre[r2][c2] - self.pre[r1][c2] - self.pre[r2][c1] + self.pre[r1][c1]
    }
}

fn main() {
    let m = NumMatrix::new(vec![
        vec![3, 0, 1, 4, 2],
        vec![5, 6, 3, 2, 1],
        vec![1, 2, 0, 1, 5],
        vec![4, 1, 0, 1, 7],
        vec![1, 0, 3, 0, 5],
    ]);
    println!("{}", m.sum_region(2, 1, 4, 3));
}

#[cfg(test)]
mod tests {
    use super::NumMatrix;

    #[test]
    fn example() {
        let m = NumMatrix::new(vec![
            vec![3, 0, 1, 4, 2],
            vec![5, 6, 3, 2, 1],
            vec![1, 2, 0, 1, 5],
            vec![4, 1, 0, 1, 7],
            vec![1, 0, 3, 0, 5],
        ]);
        assert_eq!(m.sum_region(2, 1, 4, 3), 8);
        assert_eq!(m.sum_region(1, 1, 2, 2), 11);
    }
}
