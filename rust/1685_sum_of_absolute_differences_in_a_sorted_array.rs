/// LeetCode #1685 - Sum Of Absolute Differences In A Sorted Array
fn get_sum_absolute_differences(nums: Vec<i32>) -> Vec<i32> {
    let n = nums.len();
    let total: i64 = nums.iter().map(|&x| x as i64).sum();
    let mut pref = 0i64;
    let mut ans = vec![0i32; n];
    for i in 0..n {
        let left = pref;
        let right = total - pref - nums[i] as i64;
        ans[i] = (nums[i] as i64 * i as i64 - left + right - nums[i] as i64 * (n - i - 1) as i64) as i32;
        pref += nums[i] as i64;
    }
    ans
}
fn main() { println!("{:?}", get_sum_absolute_differences(vec![2,3,5])); }
#[cfg(test)]
mod tests {
    use super::get_sum_absolute_differences;
    #[test]
    fn example_one() { assert_eq!(get_sum_absolute_differences(vec![2,3,5]), vec![4,3,5]); }
}