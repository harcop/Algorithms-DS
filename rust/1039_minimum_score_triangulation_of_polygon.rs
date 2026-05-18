/// LeetCode #1039 - Minimum Score Triangulation of Polygon
fn min_score_triangulation(values: Vec<i32>) -> i32 {
    let n = values.len();
    let mut dp = vec![vec![0i32; n]; n];
    for len in 3..=n {
        for i in 0..=n - len {
            let j = i + len - 1;
            dp[i][j] = i32::MAX;
            for k in i + 1..j {
                dp[i][j] = dp[i][j].min(dp[i][k] + dp[k][j] + values[i] * values[k] * values[j]);
            }
        }
    }
    dp[0][n - 1]
}

fn main() {
    println!("{}", min_score_triangulation(vec![1, 2, 3]));
}

#[cfg(test)]
mod tests {
    use super::min_score_triangulation;

    #[test]
    fn example_one() {
        assert_eq!(min_score_triangulation(vec![1, 2, 3]), 6);
    }

    #[test]
    fn example_two() {
        assert_eq!(min_score_triangulation(vec![3, 7, 4, 5]), 144);
    }
}
