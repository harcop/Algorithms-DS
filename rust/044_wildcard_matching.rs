/// LeetCode #44 - Wildcard Matching
fn is_match(s: String, p: String) -> bool {
    let s_bytes = s.as_bytes();
    let p_bytes = p.as_bytes();
    let m = s_bytes.len();
    let n = p_bytes.len();

    let mut dp = vec![vec![false; n + 1]; m + 1];
    dp[0][0] = true;

    for j in 1..=n {
        if p_bytes[j - 1] == b'*' {
            dp[0][j] = dp[0][j - 1];
        }
    }

    for i in 1..=m {
        for j in 1..=n {
            if p_bytes[j - 1] == b'*' {
                dp[i][j] = dp[i][j - 1] || dp[i - 1][j];
            } else if p_bytes[j - 1] == b'?' || p_bytes[j - 1] == s_bytes[i - 1] {
                dp[i][j] = dp[i - 1][j - 1];
            }
        }
    }

    dp[m][n]
}

fn main() {
    println!("{}", is_match("adceb".to_string(), "*a*b".to_string()));
}

#[cfg(test)]
mod tests {
    use super::is_match;

    #[test]
    fn example_one() {
        assert!(!is_match("aa".to_string(), "a".to_string()));
    }

    #[test]
    fn example_two() {
        assert!(is_match("aa".to_string(), "*".to_string()));
    }

    #[test]
    fn example_three() {
        assert!(!is_match("cb".to_string(), "?a".to_string()));
    }
}
