/// LeetCode #931 - Minimum Falling Path Sum
fn min_falling_path_sum(matrix: Vec<Vec<i32>>) -> i32 {
    let n = matrix.len();
    let mut dp = matrix[n - 1].clone();
    for r in (0..n - 1).rev() {
        let mut ndp = vec![0i32; n];
        for c in 0..n {
            let mut best = dp[c];
            if c > 0 {
                best = best.min(dp[c - 1]);
            }
            if c + 1 < n {
                best = best.min(dp[c + 1]);
            }
            ndp[c] = matrix[r][c] + best;
        }
        dp = ndp;
    }
    *dp.iter().min().unwrap()
}

fn main() {
    let m = vec![vec![2, 1, 3], vec![6, 5, 4], vec![7, 8, 9]];
    println!("{}", min_falling_path_sum(m));
}

#[cfg(test)]
mod tests {
    use super::min_falling_path_sum;

    #[test]
    fn example_one() {
        let m = vec![vec![2, 1, 3], vec![6, 5, 4], vec![7, 8, 9]];
        assert_eq!(min_falling_path_sum(m), 13);
    }

    #[test]
    fn example_two() {
        assert_eq!(min_falling_path_sum(vec![vec![-19, 57], vec![-40, -5]]), -59);
    }
}
