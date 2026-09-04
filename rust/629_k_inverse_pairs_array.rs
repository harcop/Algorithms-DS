/// LeetCode #629 - K Inverse Pairs Array
fn k_inverse_pairs(n: i32, k: i32) -> i32 {
    const MOD: i64 = 1_000_000_007;
    let n = n as usize;
    let k = k as usize;
    let mut dp = vec![vec![0i64; k + 1]; n + 1];
    dp[0][0] = 1;
    for i in 1..=n {
        dp[i][0] = 1;
        for j in 1..=k {
            let mut val = dp[i][j - 1] + dp[i - 1][j];
            if j >= i {
                val -= dp[i - 1][j - i];
            }
            dp[i][j] = (val % MOD + MOD) % MOD;
        }
    }
    dp[n][k] as i32
}

fn main() {
    println!("{}", k_inverse_pairs(3, 0));
}

#[cfg(test)]
mod tests {
    use super::k_inverse_pairs;

    #[test]
    fn example_one() {
        assert_eq!(k_inverse_pairs(3, 0), 1);
    }

    #[test]
    fn example_two() {
        assert_eq!(k_inverse_pairs(3, 1), 2);
    }
}
