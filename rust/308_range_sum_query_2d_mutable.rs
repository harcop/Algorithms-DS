/// LeetCode #308 - Range Sum Query 2D - Mutable (recompute rectangle sum from stored matrix)
pub struct NumMatrix {
    g: Vec<Vec<i32>>,
}

impl NumMatrix {
    fn new(matrix: Vec<Vec<i32>>) -> Self {
        NumMatrix { g: matrix }
    }

    fn update(&mut self, row: i32, col: i32, val: i32) {
        self.g[row as usize][col as usize] = val;
    }

    fn sum_region(&self, row1: i32, col1: i32, row2: i32, col2: i32) -> i32 {
        let mut s = 0i32;
        for i in row1..=row2 {
            for j in col1..=col2 {
                s += self.g[i as usize][j as usize];
            }
        }
        s
    }
}

fn main() {
    let m = NumMatrix::new(vec![vec![3, 0, 1, 4, 2], vec![5, 6, 3, 2, 1]]);
    println!("{}", m.sum_region(0, 0, 1, 2));
}

#[cfg(test)]
mod tests {
    use super::NumMatrix;

    #[test]
    fn example() {
        let mut m = NumMatrix::new(vec![
            vec![3, 0, 1, 4, 2],
            vec![5, 6, 3, 2, 1],
            vec![1, 2, 0, 1, 5],
            vec![4, 1, 0, 1, 7],
            vec![1, 0, 3, 0, 5],
        ]);
        assert_eq!(m.sum_region(2, 1, 4, 3), 8);
        m.update(3, 2, 2);
        assert_eq!(m.sum_region(2, 1, 4, 3), 10);
    }
}
