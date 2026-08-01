/// LeetCode #2863 - Maximum Length of Semi-Decreasing Subarrays
fn max_subarray_length(nums: Vec<i32>) -> i32 {
    use std::collections::BTreeMap;

    let mut positions: BTreeMap<i32, Vec<usize>> = BTreeMap::new();
    for (i, value) in nums.into_iter().enumerate() {
        positions.entry(value).or_default().push(i);
    }

    let mut answer = 0;
    let mut earliest_greater = usize::MAX;
    for indices in positions.values().rev() {
        if earliest_greater <= indices[indices.len() - 1] {
            answer = answer.max(indices[indices.len() - 1] - earliest_greater + 1);
        }
        earliest_greater = earliest_greater.min(indices[0]);
    }
    answer as i32
}

fn main() {
    println!(
        "{}",
        max_subarray_length(vec![7, 6, 5, 4, 3, 2, 1, 6, 10, 11])
    );
}

#[cfg(test)]
mod tests {
    use super::max_subarray_length;

    #[test]
    fn example_one() {
        assert_eq!(
            max_subarray_length(vec![7, 6, 5, 4, 3, 2, 1, 6, 10, 11]),
            8
        );
    }

    #[test]
    fn example_two() {
        assert_eq!(
            max_subarray_length(vec![57, 55, 50, 60, 61, 58, 63, 59, 64, 60, 63]),
            6
        );
    }

    #[test]
    fn example_three() {
        assert_eq!(max_subarray_length(vec![1, 2, 3, 4]), 0);
    }
}
