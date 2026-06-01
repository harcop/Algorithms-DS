/// LeetCode #1696 - Jump Game Vi
use std::collections::VecDeque;

fn max_result(nums: Vec<i32>, k: i32) -> i32 {
    let k = k as usize;
    let n = nums.len();
    let mut dp = vec![0i32; n];
    dp[0] = nums[0];
    let mut dq = VecDeque::from([0usize]);
    for i in 1..n {
        while dq.front().copied().unwrap_or(0) < i.saturating_sub(k) { dq.pop_front(); }
        dp[i] = nums[i] + dp[dq[0]];
        while dq.back().copied().map(|j| dp[j] <= dp[i]).unwrap_or(false) { dq.pop_back(); }
        dq.push_back(i);
    }
    dp[n - 1]
}
fn main() { println!("{}", max_result(vec![1,-1,-2,4,-7,3], 2)); }
#[cfg(test)]
mod tests {
    use super::max_result;
    #[test]
    fn example_one() { assert_eq!(max_result(vec![1,-1,-2,4,-7,3], 2), 7); }
}