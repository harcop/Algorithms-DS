/// LeetCode #1425 - Constrained Subsequence Sum
use std::collections::VecDeque;

fn constrained_subset_sum(nums: Vec<i32>, k: i32) -> i32 {
    let n = nums.len();
    let k = k as usize;
    let mut dp = vec![0i32; n];
    let mut dq = VecDeque::new();
    let mut ans = i32::MIN;
    for i in 0..n {
        while !dq.is_empty() && dq.front().unwrap() + k < i {
            dq.pop_front();
        }
        let best = if dq.is_empty() { 0 } else { dp[*dq.front().unwrap()] };
        dp[i] = nums[i].max(nums[i] + best);
        while !dq.is_empty() && dp[*dq.back().unwrap()] <= dp[i] {
            dq.pop_back();
        }
        dq.push_back(i);
        ans = ans.max(dp[i]);
    }
    ans
}

fn main() {
    println!("{}", constrained_subset_sum(vec![10, 2, -10, 5, 20], 2));
}

#[cfg(test)]
mod tests {
    use super::constrained_subset_sum;

    #[test]
    fn example_one() {
        assert_eq!(constrained_subset_sum(vec![10, 2, -10, 5, 20], 2), 37);
    }

    #[test]
    fn example_two() {
        assert_eq!(constrained_subset_sum(vec![-1, -2, -3], 1), -1);
    }
}

