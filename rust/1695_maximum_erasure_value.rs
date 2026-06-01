/// LeetCode #1695 - Maximum Erasure Value
use std::collections::HashSet;

fn maximum_unique_subarray(nums: Vec<i32>) -> i32 {
    let mut set = HashSet::new();
    let mut l = 0usize;
    let mut sum = 0i32;
    let mut ans = 0i32;
    for (r, &x) in nums.iter().enumerate() {
        while !set.insert(x) {
            sum -= nums[l];
            set.remove(&nums[l]);
            l += 1;
        }
        sum += x;
        ans = ans.max(sum);
    }
    ans
}
fn main() { println!("{}", maximum_unique_subarray(vec![4,2,4,5,6])); }
#[cfg(test)]
mod tests {
    use super::maximum_unique_subarray;
    #[test]
    fn example_one() { assert_eq!(maximum_unique_subarray(vec![4,2,4,5,6]), 17); }
}