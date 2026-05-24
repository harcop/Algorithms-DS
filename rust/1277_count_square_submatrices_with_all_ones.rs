/// LeetCode #1277 - Count Square Submatrices with All Ones
fn count_squares(matrix: Vec<Vec<i32>>) -> i32 {
    let m = matrix.len();
    if m == 0 {
        return 0;
    }
    let n = matrix[0].len();
    let mut dp = vec![vec![0; n]; m];
    let mut ans = 0;
    for i in 0..m {
        for j in 0..n {
            if matrix[i][j] == 1 {
                if i == 0 || j == 0 {
                    dp[i][j] = 1;
                } else {
                    dp[i][j] = 1 + dp[i - 1][j].min(dp[i][j - 1]).min(dp[i - 1][j - 1]);
                }
                ans += dp[i][j];
            }
        }
    }
    ans
}

fn main() {
    println!(
        "{}",
        count_squares(vec![
            vec![0, 1, 1, 1],
            vec![1, 1, 1, 1],
            vec![0, 1, 1, 1],
        ])
    );
}

#[cfg(test)]
mod tests {
    use super::count_squares;

    #[test]
    fn example_one() {
        assert_eq!(
            count_squares(vec![
                vec![0, 1, 1, 1],
                vec![1, 1, 1, 1],
                vec![0, 1, 1, 1],
            ]),
            15
        );
    }

    #[test]
    fn example_two() {
        assert_eq!(
            count_squares(vec![vec![1, 0, 1], vec![1, 1, 0], vec![1, 1, 1]]),
            7
        );
    }
}
