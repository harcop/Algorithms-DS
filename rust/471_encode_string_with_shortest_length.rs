/// LeetCode #471 - Encode String with Shortest Length
fn encode(s: String) -> String {
    let n = s.len();
    if n == 0 {
        return s;
    }
    let bytes = s.as_bytes();
    let mut dp = vec![vec![String::new(); n]; n];
    for i in 0..n {
        dp[i][i] = (bytes[i] as char).to_string();
    }
    for len in 2..=n {
        for i in 0..=n - len {
            let j = i + len - 1;
            let sub = std::str::from_utf8(&bytes[i..=j]).unwrap();
            let mut best = sub.to_string();
            for k in 1..=len / 2 {
                if len % k == 0 && is_repeat(&bytes[i..=j], k) {
                    let enc = format!("{}[{}]", len / k, dp[i][i + k - 1]);
                    if enc.len() < best.len() {
                        best = enc;
                    }
                }
            }
            for k in i..j {
                if dp[i][k].len() + dp[k + 1][j].len() < best.len() {
                    best = format!("{}{}", dp[i][k], dp[k + 1][j]);
                }
            }
            dp[i][j] = best;
        }
    }
    dp[0][n - 1].clone()
}

fn is_repeat(s: &[u8], k: usize) -> bool {
    s.chunks(k).all(|c| c == &s[..k])
}

fn main() {
    println!("{}", encode("aaaaa".into()));
}

#[cfg(test)]
mod tests {
    use super::encode;

    #[test]
    fn example_one() {
        assert_eq!(encode("aaa".into()), "aaa");
    }

    #[test]
    fn example_two() {
        assert_eq!(encode("aaaaa".into()), "5[a]");
    }

    #[test]
    fn example_three() {
        assert_eq!(encode("aaaaaaaaaa".into()), "10[a]");
    }

    #[test]
    fn example_four() {
        assert_eq!(encode("aabcaabcd".into()), "2[aabc]d");
    }

    #[test]
    fn example_five() {
        assert_eq!(encode("abbbabbbcabbbabbbc".into()), "2[2[abbb]c]");
    }
}
