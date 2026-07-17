/// LeetCode #2453 - Destroy Sequential Targets
use std::collections::HashMap;

fn destroy_targets(nums: Vec<i32>, space: i32) -> i32 {
    let mut counts = HashMap::new();
    for &num in &nums {
        *counts.entry(num % space).or_insert(0) += 1;
    }

    let best_count = counts.values().copied().max().unwrap();
    nums.into_iter()
        .filter(|num| counts[&(num % space)] == best_count)
        .min()
        .unwrap()
}

fn main() {
    println!("{}", destroy_targets(vec![3, 7, 8, 1, 1, 5], 2));
}

#[cfg(test)]
mod tests {
    use super::destroy_targets;

    #[test]
    fn example_one() {
        assert_eq!(destroy_targets(vec![3, 7, 8, 1, 1, 5], 2), 1);
    }

    #[test]
    fn example_two() {
        assert_eq!(destroy_targets(vec![1, 3, 5, 2, 4, 6], 2), 1);
    }
}
