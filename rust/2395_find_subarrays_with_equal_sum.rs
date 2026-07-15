/// LeetCode #2395 - Find Subarrays With Equal Sum
use std::collections::HashSet;

fn find_subarrays(nums: Vec<i32>) -> bool {
    let mut seen = HashSet::new();
    for i in 1..nums.len() {
        if !seen.insert(nums[i - 1] + nums[i]) {
            return true;
        }
    }
    false
}

fn main() {
    println!("{}", find_subarrays(vec![4, 2, 4]));
}

#[cfg(test)]
mod tests {
    use super::find_subarrays;

    #[test]
    fn example_one() {
        assert!(find_subarrays(vec![4, 2, 4]));
    }

    #[test]
    fn example_two() {
        assert!(!find_subarrays(vec![1, 2, 3, 4, 5]));
    }

    #[test]
    fn example_three() {
        assert!(find_subarrays(vec![0, 0, 0]));
    }
}
