/// LeetCode #1639 - Number Of Ways To Form A Target String Given A Dictionary
const MOD: i64 = 1_000_000_007;

fn num_ways(words: Vec<String>, target: String) -> i32 {
    let wlen = words[0].len();
    let mut cnt = vec![vec![0i64; 26]; wlen];
    for w in &words {
        for (j, &c) in w.as_bytes().iter().enumerate() {
            cnt[j][(c - b'a') as usize] += 1;
        }
    }
    let t = target.as_bytes();
    let m = t.len();
    let mut dp = vec![vec![0i64; wlen + 1]; m + 1];
    for j in 0..=wlen { dp[m][j] = 1; }
    for i in (0..m).rev() {
        for j in (0..wlen).rev() {
            dp[i][j] = dp[i][j + 1];
            dp[i][j] = (dp[i][j] + dp[i + 1][j + 1] * cnt[j][(t[i] - b'a') as usize]) % MOD;
        }
    }
    dp[0][0] as i32
}
fn main() { println!("{}", num_ways(vec!["acca".into(),"bbbb".into(),"caca".into()], "aba".into())); }
#[cfg(test)]
mod tests {
    use super::num_ways;
    #[test]
    fn example_one() { assert_eq!(num_ways(vec!["acca".into(),"bbbb".into(),"caca".into()], "aba".into()), 6); }
}