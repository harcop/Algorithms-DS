/// LeetCode #3448 - Count Substrings Divisible By Last Digit
fn count_substrings(s: String) -> i64 {
    let mut ans = 0i64;
    let mut dp = vec![vec![0i64; 10]; 10];
    for c in s.bytes() {
        let digit = (c - b'0') as usize;
        let mut new_dp = vec![vec![0i64; 10]; 10];
        for num in 1..10 {
            for rem in 0..num {
                new_dp[num][(rem * 10 + digit) % num] += dp[num][rem];
            }
            new_dp[num][digit % num] += 1;
        }
        dp = new_dp;
        ans += dp[digit][0];
    }
    ans
}

fn main() {
    println!("{}", count_substrings("12936".into()));
}

#[cfg(test)]
mod tests {
    use super::count_substrings;

    #[test]
    fn example1() {
        assert_eq!(count_substrings("12936".into()), 11);
    }

    #[test]
    fn example2() {
        assert_eq!(count_substrings("5701283".into()), 18);
    }

    #[test]
    fn example3() {
        assert_eq!(count_substrings("1010101010".into()), 25);
    }
}
