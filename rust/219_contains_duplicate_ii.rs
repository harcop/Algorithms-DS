/// LeetCode #219 - Contains Duplicate II
use std::collections::HashMap;

fn contains_nearby_duplicate(nums: Vec<i32>, k: i32) -> bool {
    let k = k as usize;
    let mut last: HashMap<i32, usize> = HashMap::new();
    for (i, &x) in nums.iter().enumerate() {
        if let Some(&j) = last.get(&x) {
            if i - j <= k {
                return true;
            }
        }
        last.insert(x, i);
    }
    false
}

fn main() {
    println!("{}", contains_nearby_duplicate(vec![1, 2, 3, 1], 3));
}

#[cfg(test)]
mod tests {
    use super::contains_nearby_duplicate;

    #[test]
    fn example_one() {
        assert!(contains_nearby_duplicate(vec![1, 2, 3, 1], 3));
    }

    #[test]
    fn example_two() {
        assert!(contains_nearby_duplicate(vec![1, 0, 1, 1], 1));
    }

    #[test]
    fn example_three() {
        assert!(!contains_nearby_duplicate(vec![1, 2, 3, 1, 2, 3], 2));
    }
}
