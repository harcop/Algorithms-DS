/// LeetCode #3473 - Sum of K Subarrays With Length at Least M
fn max_sum(nums: Vec<i32>, k: i32, m: i32) -> i32 {
    let n = nums.len();
    let k = k as usize;
    let m = m as usize;
    let mut prefix = vec![0i64; n + 1];
    for i in 0..n {
        prefix[i + 1] = prefix[i] + nums[i] as i64;
    }
    const NEG: i64 = i64::MIN / 4;
    let mut dp = vec![NEG; n + 1];
    dp[0] = 0;
    for t in 1..=k {
        let mut new_dp = vec![NEG; n + 1];
        let mut mx = NEG;
        for j in t * m - 1..n {
            mx = mx.max(dp[j + 1 - m]);
            new_dp[j + 1] = prefix[j + 1] - prefix[j + 1 - m] + mx;
            if j + 1 != t * m {
                new_dp[j + 1] = new_dp[j + 1].max(new_dp[j] + nums[j] as i64);
            }
        }
        dp = new_dp;
    }
    *dp.iter().max().unwrap() as i32
}

fn main() {
    println!("{}", max_sum(vec![1, 2, -1, 3, 3, 4], 2, 2));
}

#[cfg(test)]
mod tests {
    use super::max_sum;

    #[test]
    fn example1() {
        assert_eq!(max_sum(vec![1, 2, -1, 3, 3, 4], 2, 2), 13);
    }

    #[test]
    fn example2() {
        assert_eq!(max_sum(vec![-10, 3, -1, -2], 4, 1), -10);
    }
}
