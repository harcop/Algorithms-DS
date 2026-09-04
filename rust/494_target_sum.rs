/// LeetCode #494 - Target Sum
fn find_target_sum_ways(nums: Vec<i32>, target: i32) -> i32 {
    let sum: i32 = nums.iter().sum();
    if target.abs() > sum || (sum + target) % 2 != 0 {
        return 0;
    }
    let want = ((sum + target) / 2) as usize;
    let mut dp = vec![0i32; want + 1];
    dp[0] = 1;
    for x in nums {
        let x = x as usize;
        for s in (x..=want).rev() {
            dp[s] += dp[s - x];
        }
    }
    dp[want]
}

fn main() {
    println!("{}", find_target_sum_ways(vec![1, 1, 1, 1, 1], 3));
}

#[cfg(test)]
mod tests {
    use super::find_target_sum_ways;

    #[test]
    fn example_one() {
        assert_eq!(find_target_sum_ways(vec![1, 1, 1, 1, 1], 3), 5);
    }

    #[test]
    fn example_two() {
        assert_eq!(find_target_sum_ways(vec![1], 1), 1);
    }
}
