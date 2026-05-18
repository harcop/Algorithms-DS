/// LeetCode #879 - Profitable Schemes
fn profitable_schemes(n: i32, min_profit: i32, group: Vec<i32>, profit: Vec<i32>) -> i32 {
    const MOD: i64 = 1_000_000_007;
    let n = n as usize;
    let mp = min_profit as usize;
    let mut dp = vec![vec![0i64; mp + 1]; n + 1];
    dp[0][0] = 1;

    for (&g_raw, &p_raw) in group.iter().zip(profit.iter()) {
        let g = g_raw as usize;
        let pf = p_raw as usize;
        for j in (g..=n).rev() {
            for pr in 0..=mp {
                let prev = dp[j - g][pr];
                if prev == 0 {
                    continue;
                }
                let np = pr + pf;
                if np >= mp {
                    dp[j][mp] = (dp[j][mp] + prev) % MOD;
                } else {
                    dp[j][np] = (dp[j][np] + prev) % MOD;
                }
            }
        }
    }

    let mut ans = 0i64;
    for j in 0..=n {
        ans = (ans + dp[j][mp]) % MOD;
    }
    ans as i32
}

fn main() {
    println!("{}", profitable_schemes(5, 3, vec![2, 2], vec![2, 3]));
}

#[cfg(test)]
mod tests {
    use super::profitable_schemes;

    #[test]
    fn example_one() {
        assert_eq!(profitable_schemes(5, 3, vec![2, 2], vec![2, 3]), 2);
    }
}
