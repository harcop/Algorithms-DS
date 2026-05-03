/// LeetCode #221 - Maximal Square
fn maximal_square(matrix: Vec<Vec<char>>) -> i32 {
    let m = matrix.len();
    if m == 0 {
        return 0;
    }
    let n = matrix[0].len();
    let mut dp = vec![vec![0; n + 1]; m + 1];
    let mut best = 0;
    for i in 1..=m {
        for j in 1..=n {
            if matrix[i - 1][j - 1] == '1' {
                dp[i][j] = dp[i - 1][j]
                    .min(dp[i][j - 1])
                    .min(dp[i - 1][j - 1])
                    + 1;
                best = best.max(dp[i][j]);
            }
        }
    }
    (best * best) as i32
}

fn main() {
    println!(
        "{}",
        maximal_square(vec![
            vec!['1', '0', '1', '0', '0'],
            vec!['1', '0', '1', '1', '1'],
            vec!['1', '1', '1', '1', '1'],
            vec!['1', '0', '0', '1', '0'],
        ])
    );
}

#[cfg(test)]
mod tests {
    use super::maximal_square;

    #[test]
    fn example_one() {
        let m = vec![
            vec!['1', '0', '1', '0', '0'],
            vec!['1', '0', '1', '1', '1'],
            vec!['1', '1', '1', '1', '1'],
            vec!['1', '0', '0', '1', '0'],
        ];
        assert_eq!(maximal_square(m), 4);
    }

    #[test]
    fn example_two() {
        assert_eq!(maximal_square(vec![vec!['0']]), 0);
    }

    #[test]
    fn example_three() {
        assert_eq!(maximal_square(vec![vec!['1']]), 1);
    }
}
