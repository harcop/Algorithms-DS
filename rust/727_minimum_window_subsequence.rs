/// LeetCode #727 - Minimum Window Subsequence
fn min_window(s1: String, t: String) -> String {
    let s = s1.as_bytes();
    let t = t.as_bytes();
    let n = t.len();
    if n == 0 {
        return String::new();
    }
    let mut dp = vec![-1i32; n];
    let mut best: Option<(usize, usize)> = None;
    for (i, &c) in s.iter().enumerate() {
        for j in (0..n).rev() {
            if c == t[j] {
                if j == 0 {
                    dp[0] = i as i32;
                } else if dp[j - 1] >= 0 {
                    dp[j] = dp[j - 1];
                }
                if j == n - 1 && dp[n - 1] >= 0 {
                    let st = dp[n - 1] as usize;
                    let len = i - st + 1;
                    if best.map(|(bl, _)| len < bl).unwrap_or(true) {
                        best = Some((len, st));
                    }
                }
            }
        }
    }
    match best {
        Some((len, st)) => s1[st..st + len].to_string(),
        None => String::new(),
    }
}

fn main() {
    println!("{}", min_window("abcdebdde".into(), "bde".into()));
}

#[cfg(test)]
mod tests {
    use super::min_window;

    #[test]
    fn example_one() {
        assert_eq!(min_window("abcdebdde".into(), "bde".into()), "bcde");
    }

    #[test]
    fn example_two() {
        assert_eq!(min_window("jmeqksfrsdcmsiwbatcnmmcgdjb".into(), "k".into()), "k");
    }
}
