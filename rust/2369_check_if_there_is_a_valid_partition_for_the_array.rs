/// LeetCode #2369 - Check if There is a Valid Partition For The Array
fn valid_partition(nums: Vec<i32>) -> bool {
    let n = nums.len();
    let mut dp = vec![false; n + 1];
    dp[0] = true;
    if n >= 2 {
        dp[2] = nums[0] == nums[1];
    }

    for i in 3..=n {
        dp[i] = (dp[i - 2] && nums[i - 2] == nums[i - 1])
            || (dp[i - 3]
                && ((nums[i - 3] == nums[i - 2] && nums[i - 2] == nums[i - 1])
                    || (nums[i - 3] + 1 == nums[i - 2] && nums[i - 2] + 1 == nums[i - 1])));
    }

    dp[n]
}

fn main() {
    println!("{}", valid_partition(vec![4, 4, 4, 5, 6]));
}

#[cfg(test)]
mod tests {
    use super::valid_partition;

    #[test]
    fn example_one() {
        assert!(valid_partition(vec![4, 4, 4, 5, 6]));
    }

    #[test]
    fn example_two() {
        assert!(!valid_partition(vec![1, 1, 1, 2]));
    }
}
