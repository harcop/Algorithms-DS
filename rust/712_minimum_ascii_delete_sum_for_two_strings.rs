/// LeetCode #712 - Minimum ASCII Delete Sum for Two Strings
fn minimum_delete_sum(s1: String, s2: String) -> i32 {
    let a = s1.as_bytes();
    let b = s2.as_bytes();
    let m = a.len();
    let n = b.len();
    let mut dp = vec![vec![0i32; n + 1]; m + 1];
    for i in 1..=m {
        dp[i][0] = dp[i - 1][0] + a[i - 1] as i32;
    }
    for j in 1..=n {
        dp[0][j] = dp[0][j - 1] + b[j - 1] as i32;
    }
    for i in 1..=m {
        for j in 1..=n {
            if a[i - 1] == b[j - 1] {
                dp[i][j] = dp[i - 1][j - 1];
            } else {
                dp[i][j] = (dp[i - 1][j] + a[i - 1] as i32)
                    .min(dp[i][j - 1] + b[j - 1] as i32);
            }
        }
    }
    dp[m][n]
}

fn main() {
    println!("{}", minimum_delete_sum("sea".into(), "eat".into()));
}

#[cfg(test)]
mod tests {
    use super::minimum_delete_sum;

    #[test]
    fn example_one() {
        assert_eq!(minimum_delete_sum("sea".into(), "eat".into()), 231);
    }

    #[test]
    fn example_two() {
        assert_eq!(minimum_delete_sum("delete".into(), "leet".into()), 403);
    }
}
