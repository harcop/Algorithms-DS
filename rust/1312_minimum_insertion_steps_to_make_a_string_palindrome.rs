/// LeetCode #1312 - Minimum Insertion Steps to Make a String Palindrome
fn min_insertions(s: String) -> i32 {
    let t: Vec<char> = s.chars().rev().collect();
    let s: Vec<char> = s.chars().collect();
    let n = s.len();
    let mut dp = vec![0; n + 1];
    for &c in &t {
        let mut prev = 0;
        for j in 1..=n {
            let tmp = dp[j];
            if c == s[j - 1] {
                dp[j] = prev + 1;
            } else {
                dp[j] = dp[j].max(dp[j - 1]);
            }
            prev = tmp;
        }
    }
    (n - dp[n]) as i32
}

fn main() {
    println!("{}", min_insertions("zzazz".to_string()));
}

#[cfg(test)]
mod tests {
    use super::min_insertions;

    #[test]
    fn example_one() {
        assert_eq!(min_insertions("zzazz".to_string()), 0);
    }

    #[test]
    fn example_two() {
        assert_eq!(min_insertions("mbadm".to_string()), 2);
    }
}
