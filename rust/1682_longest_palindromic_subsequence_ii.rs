/// LeetCode #1682 - Longest Palindromic Subsequence Ii
fn longest_palindrome(s: String, k: i32) -> i32 {
    let s = s.as_bytes();
    let n = s.len();
    let mut ans = 0i32;
    for c1 in 0..26usize {
        for c2 in c1..26 {
            let mut dp = vec![0i32; n];
            for i in 0..n {
                let ch = (s[i] - b'a') as usize;
                if ch != c1 && ch != c2 {
                    continue;
                }
                dp[i] = 1;
                if i > 0
                    && ((s[i - 1] - b'a') as usize == c1 || (s[i - 1] - b'a') as usize == c2)
                {
                    dp[i] = dp[i].max(dp[i - 1] + 1);
                }
                for j in 0..i {
                    if s[i] == s[j] {
                        let inner = if j + 1 <= i - 1 { dp[j + 1] } else { 0 };
                        dp[i] = dp[i].max(inner + 2);
                    }
                }
                if dp[i] >= k {
                    ans = ans.max(dp[i]);
                }
            }
        }
    }
    ans
}

fn main() {
    println!("{}", longest_palindrome("abcccq".into(), 2));
}

#[cfg(test)]
mod tests {
    use super::longest_palindrome;

    #[test]
    fn example_one() {
        assert_eq!(longest_palindrome("abcccq".into(), 2), 4);
    }
}
