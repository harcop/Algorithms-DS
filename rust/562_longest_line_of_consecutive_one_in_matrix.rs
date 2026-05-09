/// LeetCode #562 - Longest Line of Consecutive One in Matrix
fn longest_line(mat: Vec<Vec<i32>>) -> i32 {
    let m = mat.len();
    let n = mat[0].len();
    let mut best = 0i32;
    let mut dp = vec![vec![[0i32; 4]; n]; m];
    for i in 0..m {
        for j in 0..n {
            if mat[i][j] == 0 {
                continue;
            }
            dp[i][j][0] = if j > 0 { dp[i][j - 1][0] + 1 } else { 1 };
            dp[i][j][1] = if i > 0 { dp[i - 1][j][1] + 1 } else { 1 };
            dp[i][j][2] = if i > 0 && j > 0 {
                dp[i - 1][j - 1][2] + 1
            } else {
                1
            };
            dp[i][j][3] = if i > 0 && j + 1 < n {
                dp[i - 1][j + 1][3] + 1
            } else {
                1
            };
            best = best.max(dp[i][j][0].max(dp[i][j][1]).max(dp[i][j][2]).max(dp[i][j][3]));
        }
    }
    best
}

fn main() {
    println!(
        "{}",
        longest_line(vec![vec![0, 1, 1, 0], vec![0, 1, 1, 0], vec![0, 0, 0, 1]])
    );
}

#[cfg(test)]
mod tests {
    use super::longest_line;

    #[test]
    fn example_one() {
        assert_eq!(
            longest_line(vec![vec![0, 1, 1, 0], vec![0, 1, 1, 0], vec![0, 0, 0, 1]]),
            3
        );
    }
}
