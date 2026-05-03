/// LeetCode #217 - Contains Duplicate
use std::collections::HashSet;

fn contains_duplicate(nums: Vec<i32>) -> bool {
    let mut s = HashSet::new();
    for x in nums {
        if !s.insert(x) {
            return true;
        }
    }
    false
}

fn main() {
    println!("{}", contains_duplicate(vec![1, 2, 3, 1]));
}

#[cfg(test)]
mod tests {
    use super::contains_duplicate;

    #[test]
    fn example_one() {
        assert!(contains_duplicate(vec![1, 2, 3, 1]));
    }

    #[test]
    fn example_two() {
        assert!(!contains_duplicate(vec![1, 2, 3, 4]));
    }

    #[test]
    fn example_three() {
        assert!(contains_duplicate(vec![1, 1, 1, 3, 3, 4, 3, 2, 4, 2]));
    }
}
