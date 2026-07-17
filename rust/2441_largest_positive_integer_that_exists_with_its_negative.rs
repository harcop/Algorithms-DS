/// LeetCode #2441 - Largest Positive Integer That Exists With Its Negative
use std::collections::HashSet;

fn find_max_k(nums: Vec<i32>) -> i32 {
    let values: HashSet<i32> = nums.into_iter().collect();
    values
        .iter()
        .copied()
        .filter(|&value| value > 0 && values.contains(&-value))
        .max()
        .unwrap_or(-1)
}

fn main() {
    println!("{}", find_max_k(vec![-1, 2, -3, 3]));
}

#[cfg(test)]
mod tests {
    use super::find_max_k;

    #[test]
    fn example_one() {
        assert_eq!(find_max_k(vec![-1, 2, -3, 3]), 3);
    }

    #[test]
    fn no_matching_pair() {
        assert_eq!(find_max_k(vec![-1, 10, 6, 7, -7, 1]), 7);
    }

    #[test]
    fn returns_negative_one() {
        assert_eq!(find_max_k(vec![-10, 8, 6, 7, -2, -3]), -1);
    }
}
