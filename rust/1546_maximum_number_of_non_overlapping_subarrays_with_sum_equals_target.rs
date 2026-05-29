/// LeetCode #1546 - Maximum Number Of Non Overlapping Subarrays With Sum Equals Target
use std::collections::HashSet;
fn max_non_overlapping(nums: Vec<i32>, target: i32) -> i32 {
    let mut prefix = 0i32;
    let mut seen = HashSet::new();
    seen.insert(0);
    let mut ans = 0;
    for x in nums {
        prefix += x;
        if seen.contains(&(prefix - target)) {
            ans += 1;
            prefix = 0;
            seen.clear();
            seen.insert(0);
        } else {
            seen.insert(prefix);
        }
    }
    ans
}
fn main() { println!("{}", max_non_overlapping(vec![1, 1, 1, 1, 1], 2)); }
#[cfg(test)]
mod tests {
    use super::max_non_overlapping;
    #[test]
    fn example_one() { assert_eq!(max_non_overlapping(vec![1, 1, 1, 1, 1], 2), 2); }
    #[test]
    fn example_two() { assert_eq!(max_non_overlapping(vec![-1, -2, -3, -4], -1), 1); }
}
