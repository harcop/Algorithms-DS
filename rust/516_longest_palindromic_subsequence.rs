/// LeetCode #516 - Longest Palindromic Subsequence
fn longest_palindrome_subseq(s: String) -> i32 {
    let b = s.as_bytes();
    let n = b.len();
    let mut dp = vec![vec![0; n]; n];
    for i in (0..n).rev() {
        dp[i][i] = 1;
        for j in i + 1..n {
            if b[i] == b[j] {
                dp[i][j] = dp[i + 1][j - 1] + 2;
            } else {
                dp[i][j] = dp[i + 1][j].max(dp[i][j - 1]);
            }
        }
    }
    dp[0][n - 1]
}

fn main() {
    println!("{}", longest_palindrome_subseq("bbbab".into()));
}

#[cfg(test)]
mod tests {
    use super::longest_palindrome_subseq;

    #[test]
    fn example_one() {
        assert_eq!(longest_palindrome_subseq("bbbab".into()), 4);
    }

    #[test]
    fn example_two() {
        assert_eq!(longest_palindrome_subseq("cbbd".into()), 2);
    }
}
