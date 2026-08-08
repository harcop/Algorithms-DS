/// LeetCode #3082 - Find the Sum of the Power of All Subsequences
const MOD: i64 = 1_000_000_007;

fn sum_of_power(nums: Vec<i32>, k: i32) -> i32 {
    let n = nums.len();
    let k = k as usize;
    // dp[j][s] = number of ways to pick j elements summing to s
    // Then for each subsequence with sum k of size j, it contributes 2^(n-j)
    // because remaining elements can be freely included/excluded in the outer subsequence.
    // Combined DP: f[s] = sum of powers for subsequences ending with sum s.
    // Transition: for each num, process backwards:
    // new_f[s] += f[s] (not take) + f[s-x] (take into subsequence that keeps sum)
    // Also: when we "start" contributing...
    //
    // Standard solution: dp[i][j] = ways using first i nums to get sum j.
    // Answer = sum over subsets S of nums: (# subsequences of S with sum k)
    // = sum over T subsequence with sum k: 2^(n - |T|)
    //
    // dp[j][s] = number of subsequences of length j with sum s
    let mut dp = vec![vec![0i64; k + 1]; n + 1];
    dp[0][0] = 1;
    for &x in &nums {
        let x = x as usize;
        for j in (0..n).rev() {
            for s in (0..=k).rev() {
                if dp[j][s] != 0 && s + x <= k {
                    dp[j + 1][s + x] = (dp[j + 1][s + x] + dp[j][s]) % MOD;
                }
            }
        }
    }

    let mut pow2 = vec![1i64; n + 1];
    for i in 1..=n {
        pow2[i] = (pow2[i - 1] * 2) % MOD;
    }

    let mut ans = 0i64;
    for j in 0..=n {
        ans = (ans + dp[j][k] * pow2[n - j]) % MOD;
    }
    ans as i32
}

fn main() {
    println!("{}", sum_of_power(vec![1, 2, 3], 3));
}

#[cfg(test)]
mod tests {
    use super::sum_of_power;

    #[test]
    fn example1() {
        assert_eq!(sum_of_power(vec![1, 2, 3], 3), 6);
    }

    #[test]
    fn example2() {
        assert_eq!(sum_of_power(vec![2, 3, 3], 5), 4);
    }

    #[test]
    fn example3() {
        assert_eq!(sum_of_power(vec![1, 2, 3], 7), 0);
    }
}
