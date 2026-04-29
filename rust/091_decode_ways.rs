/// LeetCode #91 - Decode Ways
fn num_decodings(s: String) -> i32 {
    let b = s.as_bytes();
    let n = b.len();
    if n == 0 || b[0] == b'0' {
        return 0;
    }
    let mut dp = vec![0i32; n + 1];
    dp[0] = 1;
    dp[1] = if b[0] != b'0' { 1 } else { 0 };

    for i in 2..=n {
        let one = if b[i - 1] != b'0' {
            dp[i - 1]
        } else {
            0
        };
        let two = {
            let v = (b[i - 2] - b'0') as i32 * 10 + (b[i - 1] - b'0') as i32;
            if (10..=26).contains(&v) {
                dp[i - 2]
            } else {
                0
            }
        };
        dp[i] = one + two;
    }
    dp[n]
}

fn main() {
    println!("{}", num_decodings("12".to_string()));
}

#[cfg(test)]
mod tests {
    use super::num_decodings;

    #[test]
    fn example_one() {
        assert_eq!(num_decodings("12".to_string()), 2);
    }

    #[test]
    fn example_two() {
        assert_eq!(num_decodings("226".to_string()), 3);
    }

    #[test]
    fn example_three() {
        assert_eq!(num_decodings("06".to_string()), 0);
    }
}
