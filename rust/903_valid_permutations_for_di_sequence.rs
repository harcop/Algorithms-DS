/// LeetCode #903 - Valid Permutations for DI Sequence
fn num_perms_di_sequence(s: String) -> i32 {
    const MOD: i64 = 1_000_000_007;
    let n = s.len();
    let mut dp = vec![vec![0i64; n + 1]; n + 1];
    dp[0][0] = 1;
    let b = s.as_bytes();
    for i in 1..=n {
        for j in 0..=i {
            if b[i - 1] == b'I' {
                for t in 0..j {
                    dp[i][j] = (dp[i][j] + dp[i - 1][t]) % MOD;
                }
            } else {
                for t in j..i {
                    dp[i][j] = (dp[i][j] + dp[i - 1][t]) % MOD;
                }
            }
        }
    }
    let mut ans = 0i64;
    for j in 0..=n {
        ans = (ans + dp[n][j]) % MOD;
    }
    ans as i32
}

fn main() {
    println!("{}", num_perms_di_sequence("DID".into()));
}

#[cfg(test)]
mod tests {
    use super::num_perms_di_sequence;

    #[test]
    fn example_one() {
        assert_eq!(num_perms_di_sequence("DID".into()), 5);
    }
}
