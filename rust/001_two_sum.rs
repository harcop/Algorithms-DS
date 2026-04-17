use std::collections::HashMap;

/// LeetCode #1 - Two Sum
///
/// Returns the indices of the two numbers such that they add up to target.
/// Assumes exactly one valid answer exists.
fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut seen: HashMap<i32, usize> = HashMap::new();

    for (index, &value) in nums.iter().enumerate() {
        let complement = target - value;

        if let Some(&prev_index) = seen.get(&complement) {
            return vec![prev_index as i32, index as i32];
        }

        seen.insert(value, index);
    }

    vec![]
}

fn main() {
    let nums = vec![2, 7, 11, 15];
    let target = 9;
    let answer = two_sum(nums, target);

    println!("{answer:?}");
}

#[cfg(test)]
mod tests {
    use super::two_sum;

    #[test]
    fn example_one() {
        assert_eq!(two_sum(vec![2, 7, 11, 15], 9), vec![0, 1]);
    }

    #[test]
    fn example_two() {
        assert_eq!(two_sum(vec![3, 2, 4], 6), vec![1, 2]);
    }

    #[test]
    fn example_three() {
        assert_eq!(two_sum(vec![3, 3], 6), vec![0, 1]);
    }
}
