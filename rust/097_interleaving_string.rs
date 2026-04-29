/// LeetCode #97 - Interleaving String
fn is_interleave(s1: String, s2: String, s3: String) -> bool {
    let a = s1.as_bytes();
    let b = s2.as_bytes();
    let c = s3.as_bytes();
    if a.len() + b.len() != c.len() {
        return false;
    }
    let mut dp = vec![vec![false; b.len() + 1]; a.len() + 1];
    dp[0][0] = true;

    for i in 0..=a.len() {
        for j in 0..=b.len() {
            if i > 0 {
                dp[i][j] |= dp[i - 1][j] && a[i - 1] == c[i + j - 1];
            }
            if j > 0 {
                dp[i][j] |= dp[i][j - 1] && b[j - 1] == c[i + j - 1];
            }
        }
    }
    dp[a.len()][b.len()]
}

fn main() {
    println!(
        "{}",
        is_interleave(
            "aabcc".to_string(),
            "dbbca".to_string(),
            "aadbbcbcac".to_string()
        )
    );
}

#[cfg(test)]
mod tests {
    use super::is_interleave;

    #[test]
    fn example_one() {
        assert!(is_interleave(
            "aabcc".to_string(),
            "dbbca".to_string(),
            "aadbbcbcac".to_string()
        ));
    }

    #[test]
    fn example_two() {
        assert!(!is_interleave(
            "aabcc".to_string(),
            "dbbca".to_string(),
            "aadbbbaccc".to_string()
        ));
    }

    #[test]
    fn example_three() {
        assert!(is_interleave("".to_string(), "".to_string(), "".to_string()));
    }
}
