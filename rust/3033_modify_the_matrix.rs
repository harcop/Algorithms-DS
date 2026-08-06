/// LeetCode #3033 - Modify the Matrix
fn modify_matrix(grid: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let n = grid.len();
    let m = grid[0].len();
    let mut col_max = vec![0; m];
    for j in 0..m {
        for i in 0..n {
            if grid[i][j] != -1 {
                col_max[j] = col_max[j].max(grid[i][j]);
            }
        }
    }
    let mut result = grid;
    for i in 0..n {
        for j in 0..m {
            if result[i][j] == -1 {
                result[i][j] = col_max[j];
            }
        }
    }
    result
}

fn main() {
    let grid = vec![vec![1, 2, -1], vec![4, -1, 6], vec![7, 8, 9]];
    println!("{:?}", modify_matrix(grid));
}

#[cfg(test)]
mod tests {
    use super::modify_matrix;

    #[test]
    fn example1() {
        assert_eq!(
            modify_matrix(vec![vec![1, 2, -1], vec![4, -1, 6], vec![7, 8, 9]]),
            vec![vec![1, 2, 9], vec![4, 8, 6], vec![7, 8, 9]]
        );
    }

    #[test]
    fn example2() {
        assert_eq!(
            modify_matrix(vec![vec![3, -1], vec![5, 2]]),
            vec![vec![3, 2], vec![5, 2]]
        );
    }
}
