/// LeetCode #2869 - Minimum Operations to Collect Elements
fn min_operations(nums: Vec<i32>, k: i32) -> i32 {
    let mut collected = vec![false; k as usize + 1];
    let mut remaining = k;

    for (operations, &value) in nums.iter().rev().enumerate() {
        if value <= k && !collected[value as usize] {
            collected[value as usize] = true;
            remaining -= 1;
        }
        if remaining == 0 {
            return operations as i32 + 1;
        }
    }
    nums.len() as i32
}

fn main() {
    println!("{}", min_operations(vec![3, 1, 5, 4, 2], 2));
}

#[cfg(test)]
mod tests {
    use super::min_operations;

    #[test]
    fn example_one() {
        assert_eq!(min_operations(vec![3, 1, 5, 4, 2], 2), 4);
    }

    #[test]
    fn example_two() {
        assert_eq!(min_operations(vec![3, 1, 5, 4, 2], 5), 5);
    }

    #[test]
    fn example_three() {
        assert_eq!(min_operations(vec![3, 2, 5, 3, 1], 3), 4);
    }
}
