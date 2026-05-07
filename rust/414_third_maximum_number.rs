/// LeetCode #414 - Third Maximum Number
use std::collections::BTreeSet;

fn third_max(nums: Vec<i32>) -> i32 {
    let mut set: BTreeSet<i32> = nums.into_iter().collect();
    let mut v: Vec<i32> = set.into_iter().rev().collect();
    if v.len() >= 3 {
        v[2]
    } else {
        v[0]
    }
}

fn main() {
    println!("{}", third_max(vec![3, 2, 1]));
}

#[cfg(test)]
mod tests {
    use super::third_max;

    #[test]
    fn example_one() {
        assert_eq!(third_max(vec![3, 2, 1]), 1);
    }

    #[test]
    fn example_two() {
        assert_eq!(third_max(vec![1, 2]), 2);
    }
}
