/// LeetCode #1074 - Number of Submatrices That Sum to Target
fn num_submatrix_sum_target(matrix: Vec<Vec<i32>>, target: i32) -> i32 {
    use std::collections::HashMap;
    let n = matrix.len();
    let m = matrix[0].len();
    let mut ans = 0i32;
    for top in 0..n {
        let mut col = vec![0i32; m];
        for bottom in top..n {
            for j in 0..m {
                col[j] += matrix[bottom][j];
            }
            let mut prefix = 0i32;
            let mut cnt: HashMap<i32, i32> = HashMap::new();
            cnt.insert(0, 1);
            for &x in &col {
                prefix += x;
                ans += cnt.get(&(prefix - target)).copied().unwrap_or(0);
                *cnt.entry(prefix).or_default() += 1;
            }
        }
    }
    ans
}

fn main() {
    println!("{}", num_submatrix_sum_target(vec![vec![0, 1, 0], vec![1, 1, 1], vec![0, 1, 0]], 0));
}

#[cfg(test)]
mod tests {
    use super::num_submatrix_sum_target;

    #[test]
    fn example_one() {
        assert_eq!(num_submatrix_sum_target(vec![vec![0, 1, 0], vec![1, 1, 1], vec![0, 1, 0]], 0), 4);
    }

    #[test]
    fn example_two() {
        assert_eq!(num_submatrix_sum_target(vec![vec![1, -1], vec![-1, 1]], 0), 5);
    }
}
