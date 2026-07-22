/// LeetCode #2567 - Minimum Score by Changing Two Elements
fn minimize_sum(mut nums: Vec<i32>) -> i32 {
    nums.sort_unstable();
    let n = nums.len();
    (nums[n - 1] - nums[2])
        .min(nums[n - 2] - nums[1])
        .min(nums[n - 3] - nums[0])
}

fn main() {
    println!("{}", minimize_sum(vec![1, 4, 7, 8, 5]));
}

#[cfg(test)]
mod tests {
    use super::minimize_sum;

    #[test]
    fn example_one() {
        assert_eq!(minimize_sum(vec![1, 4, 7, 8, 5]), 3);
    }

    #[test]
    fn example_two() {
        assert_eq!(minimize_sum(vec![1, 4, 3]), 0);
    }
}
