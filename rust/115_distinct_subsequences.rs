/// LeetCode #115 - Distinct Subsequences
fn num_distinct(s: String, t: String) -> i32 {
    let s = s.as_bytes();
    let t = t.as_bytes();
    let m = s.len();
    let n = t.len();
    if n == 0 {
        return 1;
    }
    if m < n {
        return 0;
    }

    let mut dp = vec![0i64; n + 1];
    dp[0] = 1;

    for i in 0..m {
        for j in (1..=n).rev() {
            if s[i] == t[j - 1] {
                dp[j] += dp[j - 1];
            }
        }
    }

    dp[n] as i32
}

fn main() {
    println!("{}", num_distinct("rabbbit".to_string(), "rabbit".to_string()));
}

#[cfg(test)]
mod tests {
    use super::num_distinct;

    #[test]
    fn example_one() {
        assert_eq!(num_distinct("rabbbit".to_string(), "rabbit".to_string()), 3);
    }

    #[test]
    fn example_two() {
        assert_eq!(num_distinct("babgbag".to_string(), "bag".to_string()), 5);
    }
}
