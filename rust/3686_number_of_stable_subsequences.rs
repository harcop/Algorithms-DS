/// LeetCode #3686 - Number of Stable Subsequences
fn count_stable_subsequences(nums: Vec<i32>) -> i32 {
    const MOD: i64 = 1_000_000_007;
    let mut dp = [[0i64; 2]; 2];
    for x in nums {
        let p = (x % 2) as usize;
        dp[p][1] = (dp[p][1] + dp[p][0]) % MOD;
        dp[p][0] = (dp[p][0] + 1 + dp[1 ^ p][0] + dp[1 ^ p][1]) % MOD;
    }
    ((dp[0][0] + dp[0][1] + dp[1][0] + dp[1][1]) % MOD) as i32
}

fn main() {
    println!("{}", count_stable_subsequences(vec![1, 3, 5]));
}

#[cfg(test)]
mod tests {
    use super::count_stable_subsequences;

    #[test]
    fn example1() {
        assert_eq!(count_stable_subsequences(vec![1, 3, 5]), 6);
    }

    #[test]
    fn example2() {
        assert_eq!(count_stable_subsequences(vec![2, 3, 4, 2]), 14);
    }
}
