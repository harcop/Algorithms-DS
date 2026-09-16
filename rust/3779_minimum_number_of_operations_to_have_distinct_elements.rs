/// LeetCode #3779 - Minimum Number of Operations to Have Distinct Elements
use std::collections::HashSet;

fn min_operations(nums: Vec<i32>) -> i32 {
    let mut st = HashSet::new();
    for i in (0..nums.len()).rev() {
        if !st.insert(nums[i]) {
            return (i / 3) as i32 + 1;
        }
    }
    0
}

fn main() {
    println!("{}", min_operations(vec![3, 8, 3, 6, 5, 8]));
}

#[cfg(test)]
mod tests {
    use super::min_operations;

    #[test]
    fn example1() {
        assert_eq!(min_operations(vec![3, 8, 3, 6, 5, 8]), 1);
    }

    #[test]
    fn example2() {
        assert_eq!(min_operations(vec![2, 2]), 1);
    }

    #[test]
    fn example3() {
        assert_eq!(min_operations(vec![4, 3, 5, 1, 2]), 0);
    }
}
