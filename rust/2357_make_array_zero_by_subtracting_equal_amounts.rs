/// LeetCode #2357 - Make Array Zero by Subtracting Equal Amounts
use std::collections::HashSet;

fn minimum_operations(nums: Vec<i32>) -> i32 {
    let mut s: HashSet<i32> = nums.into_iter().collect();
    s.remove(&0);
    s.len() as i32
}

fn main() {
    println!("{}", minimum_operations(vec![1, 5, 0, 3, 5]));
}

#[cfg(test)]
mod tests {
    use super::minimum_operations;

    #[test]
    fn example_one() {
        assert_eq!(minimum_operations(vec![1, 5, 0, 3, 5]), 3);
    }

    #[test]
    fn example_two() {
        assert_eq!(minimum_operations(vec![0]), 0);
    }
}
