/// LeetCode #1269 - Number of Ways to Stay in the Same Place After Some Steps
fn num_ways(steps: i32, arr_len: i32) -> i32 {
    const MOD: i64 = 1_000_000_007;
    let n = arr_len as usize;
    let steps = steps as usize;
    let mut dp = vec![vec![0i64; n]; steps + 1];
    dp[0][0] = 1;
    for s in 0..steps {
        for p in 0..n {
            let ways = dp[s][p];
            if ways == 0 {
                continue;
            }
            if p > 0 {
                dp[s + 1][p - 1] = (dp[s + 1][p - 1] + ways) % MOD;
            }
            dp[s + 1][p] = (dp[s + 1][p] + ways) % MOD;
            if p + 1 < n {
                dp[s + 1][p + 1] = (dp[s + 1][p + 1] + ways) % MOD;
            }
        }
    }
    dp[steps][0] as i32
}

fn main() {
    println!("{}", num_ways(3, 2));
}

#[cfg(test)]
mod tests {
    use super::num_ways;

    #[test]
    fn example_one() {
        assert_eq!(num_ways(3, 2), 4);
    }

    #[test]
    fn example_two() {
        assert_eq!(num_ways(3, 1), 1);
    }

    #[test]
    fn example_three() {
        assert_eq!(num_ways(6, 2), 13);
    }
}
