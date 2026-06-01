/// LeetCode #1658 - Minimum Operations To Reduce X To Zero
use std::collections::HashMap;

fn min_operations(nums: Vec<i32>, x: i32) -> i32 {
    let total: i32 = nums.iter().sum();
    let need = total - x;
    if need < 0 { return -1; }
    if need == 0 { return nums.len() as i32; }
    let mut best = -1i32;
    let mut sum = 0i32;
    let mut left = 0usize;
    for (right, &v) in nums.iter().enumerate() {
        sum += v;
        while sum > need && left <= right {
            sum -= nums[left];
            left += 1;
        }
        if sum == need { best = best.max((right - left + 1) as i32); }
    }
    if best < 0 { -1 } else { nums.len() as i32 - best }
}
fn main() { println!("{}", min_operations(vec![1,1,4,2,3], 5)); }
#[cfg(test)]
mod tests {
    use super::min_operations;
    #[test]
    fn example_one() { assert_eq!(min_operations(vec![1,1,4,2,3], 5), 2); }
    #[test]
    fn example_two() { assert_eq!(min_operations(vec![5,6,7,8,9], 4), -1); }
}