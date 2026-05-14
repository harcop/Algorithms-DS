/// LeetCode #730 - Count Different Palindromic Subsequences
const MOD: i64 = 1_000_000_007;

fn count_palindromic_subsequences(s: String) -> i32 {
    let b = s.as_bytes();
    let n = b.len();
    let mut dp = vec![vec![0i64; n]; n];
    for i in 0..n {
        dp[i][i] = 1;
    }
    for len in 2..=n {
        for i in 0..=n - len {
            let j = i + len - 1;
            if b[i] == b[j] {
                let mut l = i + 1;
                let mut r = j - 1;
                while l <= r && b[l] != b[i] {
                    l += 1;
                }
                while r >= l && b[r] != b[j] {
                    r -= 1;
                }
                if l > r {
                    dp[i][j] = dp[i + 1][j - 1] * 2 + 2;
                } else if l == r {
                    dp[i][j] = dp[i + 1][j - 1] * 2 + 1;
                } else {
                    dp[i][j] = dp[i + 1][j - 1] * 2 - dp[l + 1][r - 1];
                }
            } else {
                dp[i][j] = dp[i + 1][j] + dp[i][j - 1] - dp[i + 1][j - 1];
            }
            dp[i][j] = (dp[i][j] % MOD + MOD) % MOD;
        }
    }
    dp[0][n - 1] as i32
}

fn main() {
    println!("{}", count_palindromic_subsequences("bccb".into()));
}

#[cfg(test)]
mod tests {
    use super::count_palindromic_subsequences;

    #[test]
    fn example_one() {
        assert_eq!(count_palindromic_subsequences("bccb".into()), 6);
    }

    #[test]
    fn single_char() {
        assert_eq!(count_palindromic_subsequences("a".into()), 1);
    }
}
