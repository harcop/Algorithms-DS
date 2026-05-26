/// LeetCode #1397 - Find All Good Strings
fn find_good_strings(n: i32, s1: String, s2: String, evil: String) -> i32 {
    const MOD: i32 = 1_000_000_007;
    let n = n as usize;
    let evil_b = evil.as_bytes();
    let m = evil_b.len();

    let lps = {
        let mut l = vec![0usize; m];
        let mut len = 0usize;
        let mut i = 1usize;
        while i < m {
            if evil_b[i] == evil_b[len] {
                len += 1;
                l[i] = len;
                i += 1;
            } else if len > 0 {
                len = l[len - 1];
            } else {
                l[i] = 0;
                i += 1;
            }
        }
        l
    };

    fn count_leq(bound: &str, n: usize, evil: &[u8], lps: &[usize]) -> i32 {
        const MOD: i32 = 1_000_000_007;
        let m = evil.len();
        let b = bound.as_bytes();
        let mut dp = vec![vec![0i32; m]; 2];
        dp[1][0] = 1;
        for i in 0..n {
            let mut ndp = vec![vec![0i32; m]; 2];
            for tight in 0..2 {
                for st in 0..m {
                    let ways = dp[tight][st];
                    if ways == 0 {
                        continue;
                    }
                    let max_c = if tight == 1 && i < b.len() { b[i] } else { b'z' };
                    for c in b'a'..=max_c {
                        let ntight = if tight == 1 && i < b.len() && c == b[i] { 1 } else { 0 };
                        let mut j = st;
                        while j > 0 && evil[j] != c {
                            j = lps[j - 1];
                        }
                        if evil[j] == c {
                            j += 1;
                        }
                        if j < m {
                            ndp[ntight][j] = (ndp[ntight][j] + ways) % MOD;
                        }
                    }
                }
            }
            dp = ndp;
        }
        (dp[0].iter().sum::<i32>() + dp[1].iter().sum::<i32>()) % MOD
    }

    fn prev(s: &str) -> Option<String> {
        let mut b = s.as_bytes().to_vec();
        for i in (0..b.len()).rev() {
            if b[i] > b'a' {
                b[i] -= 1;
                for j in i + 1..b.len() {
                    b[j] = b'z';
                }
                return Some(String::from_utf8(b).unwrap());
            }
        }
        None
    }

    let hi = count_leq(&s2, n, evil_b, &lps);
    let lo = prev(&s1)
        .map(|p| count_leq(&p, n, evil_b, &lps))
        .unwrap_or(0);
    (hi - lo + MOD) % MOD
}

fn main() {
    println!("{}", find_good_strings(2, "aa".into(), "da".into(), "b".into()));
}

#[cfg(test)]
mod tests {
    use super::find_good_strings;

    #[test]
    fn example_one() {
        assert_eq!(find_good_strings(2, "aa".into(), "da".into(), "b".into()), 51);
    }

    #[test]
    fn example_two() {
        assert_eq!(
            find_good_strings(8, "leetcode".into(), "leetgoes".into(), "leet".into()),
            0
        );
    }
}
