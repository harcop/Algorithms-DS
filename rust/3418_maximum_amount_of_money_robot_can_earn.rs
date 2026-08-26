/// LeetCode #3418 - Maximum Amount of Money Robot Can Earn
fn maximum_amount(coins: Vec<Vec<i32>>) -> i32 {
    let m = coins.len();
    let n = coins[0].len();
    const NEG: i64 = i64::MIN / 4;
    let mut dp = vec![vec![vec![NEG; 3]; n]; m];
    for t in 0..3 {
        let v = coins[m - 1][n - 1] as i64;
        dp[m - 1][n - 1][t] = if t > 0 { v.max(0) } else { v };
    }
    for i in (0..m).rev() {
        for j in (0..n).rev() {
            if i == m - 1 && j == n - 1 {
                continue;
            }
            for t in 0..3 {
                let mut best = NEG;
                if i + 1 < m {
                    best = best.max(dp[i + 1][j][t]);
                }
                if j + 1 < n {
                    best = best.max(dp[i][j + 1][t]);
                }
                let v = coins[i][j] as i64;
                let mut ans = v + best;
                if v < 0 && t > 0 {
                    let mut b2 = NEG;
                    if i + 1 < m {
                        b2 = b2.max(dp[i + 1][j][t - 1]);
                    }
                    if j + 1 < n {
                        b2 = b2.max(dp[i][j + 1][t - 1]);
                    }
                    ans = ans.max(b2);
                }
                dp[i][j][t] = ans;
            }
        }
    }
    dp[0][0][2] as i32
}

fn main() {
    println!(
        "{}",
        maximum_amount(vec![vec![0, 1, -1], vec![1, -2, 3], vec![2, -3, 4]])
    );
}

#[cfg(test)]
mod tests {
    use super::maximum_amount;

    #[test]
    fn example1() {
        assert_eq!(
            maximum_amount(vec![vec![0, 1, -1], vec![1, -2, 3], vec![2, -3, 4]]),
            8
        );
    }

    #[test]
    fn example2() {
        assert_eq!(
            maximum_amount(vec![vec![10, 10, 10], vec![10, 10, 10]]),
            40
        );
    }
}
