/// LeetCode #1216 - Valid Palindrome III
fn is_valid_palindrome(s: String, k: i32) -> bool {
    let b = s.as_bytes();
    let n = b.len();
    let mut dp = vec![vec![0; n]; n];
    for i in (0..n).rev() {
        dp[i][i] = 1;
        for j in i + 1..n {
            dp[i][j] = if b[i] == b[j] {
                2 + dp[i + 1][j - 1]
            } else {
                dp[i + 1][j].max(dp[i][j - 1])
            };
        }
    }
    n as i32 - dp[0][n - 1] <= k
}

fn main() {
    println!("{}", is_valid_palindrome("abcdeca".into(), 2));
}

#[cfg(test)]
mod tests {
    use super::is_valid_palindrome;

    #[test]
    fn example_one() {
        assert!(is_valid_palindrome("abcdeca".into(), 2));
    }

    #[test]
    fn example_two() {
        assert!(is_valid_palindrome("abbababa".into(), 1));
    }
}
