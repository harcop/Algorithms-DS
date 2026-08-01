/// LeetCode #2870 - Minimum Number of Operations to Make Array Empty
fn min_operations(nums: Vec<i32>) -> i32 {
    use std::collections::HashMap;

    let mut frequencies = HashMap::new();
    for value in nums {
        *frequencies.entry(value).or_insert(0) += 1;
    }

    let mut operations = 0;
    for count in frequencies.into_values() {
        if count == 1 {
            return -1;
        }
        operations += (count + 2) / 3;
    }
    operations
}

fn main() {
    println!(
        "{}",
        min_operations(vec![2, 3, 3, 2, 2, 4, 2, 3, 4])
    );
}

#[cfg(test)]
mod tests {
    use super::min_operations;

    #[test]
    fn example_one() {
        assert_eq!(min_operations(vec![2, 3, 3, 2, 2, 4, 2, 3, 4]), 4);
    }

    #[test]
    fn example_two() {
        assert_eq!(min_operations(vec![2, 1, 2, 2, 3, 3]), -1);
    }
}
