/// LeetCode #2472 - Maximum Number of Non-overlapping Palindrome Substrings
fn max_palindromes(s: String, k: i32) -> i32 {
    let chars: Vec<u8> = s.into_bytes();
    let n = chars.len();
    let k = k as usize;
    let mut dp = vec![0; n + 1];

    let is_palindrome = |left: isize, right: isize| -> bool {
        if left < 0 {
            return false;
        }
        let mut left = left as usize;
        let mut right = right as usize;
        while left < right {
            if chars[left] != chars[right] {
                return false;
            }
            left += 1;
            right -= 1;
        }
        true
    };

    for i in k..=n {
        dp[i] = dp[i - 1];
        if is_palindrome((i - k) as isize, (i - 1) as isize) {
            dp[i] = dp[i].max(1 + dp[i - k]);
        }
        if i >= k + 1 && is_palindrome((i - k - 1) as isize, (i - 1) as isize) {
            dp[i] = dp[i].max(1 + dp[i - k - 1]);
        }
    }

    dp[n]
}

fn main() {
    println!("{}", max_palindromes("abaccdbbd".to_string(), 3));
}

#[cfg(test)]
mod tests {
    use super::max_palindromes;

    #[test]
    fn example_one() {
        assert_eq!(max_palindromes("abaccdbbd".to_string(), 3), 2);
    }

    #[test]
    fn example_two() {
        assert_eq!(max_palindromes("adbcda".to_string(), 2), 0);
    }
}
