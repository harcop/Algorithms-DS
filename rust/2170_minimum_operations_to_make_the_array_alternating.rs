/// LeetCode #2170 - Minimum Operations to Make the Array Alternating
use std::collections::HashMap;

fn minimum_operations(nums: Vec<i32>) -> i32 {
    fn top_two(nums: &[i32], start: usize) -> Vec<(i32, i32)> {
        let mut cnt = HashMap::new();
        for i in (start..nums.len()).step_by(2) {
            *cnt.entry(nums[i]).or_insert(0) += 1;
        }
        let mut pairs: Vec<(i32, i32)> = cnt.into_iter().collect();
        pairs.sort_unstable_by(|a, b| b.1.cmp(&a.1));
        pairs.resize(2, (-1, 0));
        pairs
    }

    let n = nums.len() as i32;
    let even = top_two(&nums, 0);
    let odd = top_two(&nums, 1);
    if even[0].0 != odd[0].0 {
        n - even[0].1 - odd[0].1
    } else {
        (n - even[0].1 - odd[1].1).min(n - even[1].1 - odd[0].1)
    }
}

fn main() {
    println!("{}", minimum_operations(vec![3, 1, 3, 2, 4, 3]));
}

#[cfg(test)]
mod tests {
    use super::minimum_operations;

    #[test]
    fn example_one() {
        assert_eq!(minimum_operations(vec![3, 1, 3, 2, 4, 3]), 3);
    }

    #[test]
    fn example_two() {
        assert_eq!(minimum_operations(vec![1, 2, 2, 2, 2]), 2);
    }
}
