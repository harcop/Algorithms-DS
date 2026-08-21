/// LeetCode #3336 - Find the Number of Subsequences With Equal GCD
fn gcd(mut a: i32, mut b: i32) -> i32 {
    while b != 0 {
        let t = a % b;
        a = b;
        b = t;
    }
    a
}

fn subsequence_pair_count(nums: Vec<i32>) -> i32 {
    const MOD: i64 = 1_000_000_007;
    let m = 201usize;
    let mut dp = vec![vec![0i64; m]; m];
    dp[0][0] = 1;
    for &x in &nums {
        let mut ndp = vec![vec![0i64; m]; m];
        for j in 0..m {
            for k in 0..m {
                let v = dp[j][k];
                if v == 0 {
                    continue;
                }
                ndp[j][k] = (ndp[j][k] + v) % MOD;
                let nj = gcd(j as i32, x) as usize;
                ndp[nj][k] = (ndp[nj][k] + v) % MOD;
                let nk = gcd(k as i32, x) as usize;
                ndp[j][nk] = (ndp[j][nk] + v) % MOD;
            }
        }
        dp = ndp;
    }
    let mut ans = 0i64;
    for j in 0..m {
        ans += dp[j][j];
    }
    ((ans - 1).rem_euclid(MOD)) as i32
}

fn main() {
    println!("{}", subsequence_pair_count(vec![1, 2, 3, 4]));
}

#[cfg(test)]
mod tests {
    use super::subsequence_pair_count;

    #[test]
    fn example1() {
        assert_eq!(subsequence_pair_count(vec![1, 2, 3, 4]), 10);
    }

    #[test]
    fn example2() {
        assert_eq!(subsequence_pair_count(vec![10, 20, 30]), 2);
    }

    #[test]
    fn example3() {
        assert_eq!(subsequence_pair_count(vec![1, 1, 1, 1]), 50);
    }
}
