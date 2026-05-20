/// LeetCode #1143 - Longest Common Subsequence
fn longest_common_subsequence(text1: String, text2: String) -> i32 {
    let a: Vec<u8> = text1.into_bytes();
    let b: Vec<u8> = text2.into_bytes();
    let n = a.len();
    let m = b.len();
    let mut dp = vec![0i32; m + 1];
    for i in 1..=n {
        let mut prev = 0i32;
        for j in 1..=m {
            let tmp = dp[j];
            if a[i - 1] == b[j - 1] {
                dp[j] = prev + 1;
            } else {
                dp[j] = dp[j].max(dp[j - 1]);
            }
            prev = tmp;
        }
    }
    dp[m]
}

fn main() {
    println!(
        "{}",
        longest_common_subsequence("abcde".to_string(), "ace".to_string())
    );
}

#[cfg(test)]
mod tests {
    use super::longest_common_subsequence;

    #[test]
    fn example_one() {
        assert_eq!(
            longest_common_subsequence("abcde".to_string(), "ace".to_string()),
            3
        );
    }

    #[test]
    fn example_two() {
        assert_eq!(
            longest_common_subsequence("abc".to_string(), "abc".to_string()),
            3
        );
    }
}
