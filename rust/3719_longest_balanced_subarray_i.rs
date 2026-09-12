/// LeetCode #3719 - Longest Balanced Subarray I
use std::collections::HashSet;

fn longest_balanced(nums: Vec<i32>) -> i32 {
    let mut result = 0;
    let n = nums.len();
    for left in 0..n {
        let mut curr = 0i32;
        let mut lookup = HashSet::new();
        for right in left..n {
            if lookup.insert(nums[right]) {
                curr += if nums[right] & 1 == 1 { 1 } else { -1 };
            }
            if curr == 0 {
                result = result.max((right - left + 1) as i32);
            }
        }
    }
    result
}

fn main() {
    println!("{}", longest_balanced(vec![2, 5, 4, 3]));
}

#[cfg(test)]
mod tests {
    use super::longest_balanced;

    #[test]
    fn example1() {
        assert_eq!(longest_balanced(vec![2, 5, 4, 3]), 4);
    }

    #[test]
    fn example2() {
        assert_eq!(longest_balanced(vec![3, 2, 2, 5, 4]), 5);
    }

    #[test]
    fn example3() {
        assert_eq!(longest_balanced(vec![1, 2, 3, 2]), 3);
    }
}
