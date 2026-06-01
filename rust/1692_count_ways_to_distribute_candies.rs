/// LeetCode #1692 - Count Ways To Distribute Candies
const MOD: i64 = 1_000_000_007;

fn ways_to_distribute(n: i32, k: i32) -> i32 {
    let n = n as usize;
    let k = k as usize;
    let mut dp = vec![vec![0i64; k + 1]; n + 1];
    dp[0][0] = 1;
    for i in 1..=n {
        for j in 1..=k.min(i) {
            dp[i][j] = (dp[i - 1][j - 1] + j as i64 * dp[i - 1][j]) % MOD;
        }
    }
    dp[n][k] as i32
}
fn main() { println!("{}", ways_to_distribute(3, 2)); }
#[cfg(test)]
mod tests {
    use super::ways_to_distribute;
    #[test]
    fn example_one() { assert_eq!(ways_to_distribute(3, 2), 3); }
}