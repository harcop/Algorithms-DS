/// LeetCode #3339 - Find the Number of K-Even Arrays
fn count_of_arrays(n: i32, m: i32, k: i32) -> i32 {
    const MOD: i64 = 1_000_000_007;
    let n = n as usize;
    let k = k as usize;
    let cnt0 = (m / 2) as i64;
    let cnt1 = (m - m / 2) as i64;
    let mut dp = vec![vec![vec![0i64; 2]; k + 1]; n + 1];
    dp[0][0][1] = 1;
    for i in 0..n {
        for j in 0..=k {
            for last in 0..2 {
                let ways = dp[i][j][last];
                if ways == 0 {
                    continue;
                }
                dp[i + 1][j][1] = (dp[i + 1][j][1] + ways * cnt1) % MOD;
                let nj = j + usize::from(last == 0);
                if nj <= k {
                    dp[i + 1][nj][0] = (dp[i + 1][nj][0] + ways * cnt0) % MOD;
                }
            }
        }
    }
    ((dp[n][k][0] + dp[n][k][1]) % MOD) as i32
}

fn main() {
    println!("{}", count_of_arrays(3, 4, 2));
}

#[cfg(test)]
mod tests {
    use super::count_of_arrays;

    #[test]
    fn example1() {
        assert_eq!(count_of_arrays(3, 4, 2), 8);
    }

    #[test]
    fn example2() {
        assert_eq!(count_of_arrays(5, 1, 0), 1);
    }

    #[test]
    fn example3() {
        assert_eq!(count_of_arrays(7, 7, 5), 5832);
    }
}
