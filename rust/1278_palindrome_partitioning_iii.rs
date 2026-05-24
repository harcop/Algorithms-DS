/// LeetCode #1278 - Palindrome Partitioning III
fn palindrome_partition(s: String, k: i32) -> i32 {
    let b = s.as_bytes();
    let n = b.len();
    let k = k as usize;
    let mut change = vec![vec![0; n]; n];
    for len in 1..=n {
        for i in 0..=n - len {
            let j = i + len - 1;
            change[i][j] = if len <= 1 {
                0
            } else if b[i] == b[j] {
                change[i + 1][j - 1]
            } else {
                1 + change[i + 1][j - 1]
            };
        }
    }
    let mut dp = vec![vec![i32::MAX / 2; k]; n];
    for i in 0..n {
        dp[i][0] = change[0][i];
    }
    for parts in 1..k {
        for i in parts..n {
            for j in parts..=i {
                dp[i][parts] = dp[i][parts].min(dp[j - 1][parts - 1] + change[j][i]);
            }
        }
    }
    dp[n - 1][k - 1]
}

fn main() {
    println!("{}", palindrome_partition("abc".into(), 2));
}

#[cfg(test)]
mod tests {
    use super::palindrome_partition;

    #[test]
    fn example_one() {
        assert_eq!(palindrome_partition("abc".into(), 2), 1);
    }

    #[test]
    fn example_two() {
        assert_eq!(palindrome_partition("aabbc".into(), 3), 0);
    }

    #[test]
    fn example_three() {
        assert_eq!(palindrome_partition("leetcode".into(), 8), 0);
    }
}
