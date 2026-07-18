/// LeetCode #2484 - Count Palindromic Subsequences
fn count_palindromes(s: String) -> i32 {
    const MOD: i64 = 1_000_000_007;
    let mut answer = 0i64;

    for a in b'0'..=b'9' {
        for b in b'0'..=b'9' {
            let pattern = [a, b, b'.', b, a];
            let mut dp = [0i64; 6];
            dp[5] = 1;
            for c in s.bytes() {
                for i in 0..5 {
                    if pattern[i] == b'.' || pattern[i] == c {
                        dp[i] += dp[i + 1];
                    }
                }
            }
            answer = (answer + dp[0]) % MOD;
        }
    }

    answer as i32
}

fn main() {
    println!("{}", count_palindromes("103301".to_string()));
}

#[cfg(test)]
mod tests {
    use super::count_palindromes;

    #[test]
    fn example_one() {
        assert_eq!(count_palindromes("103301".to_string()), 2);
    }

    #[test]
    fn example_two() {
        assert_eq!(count_palindromes("0000000".to_string()), 21);
    }

    #[test]
    fn example_three() {
        assert_eq!(count_palindromes("9999900000".to_string()), 2);
    }
}
