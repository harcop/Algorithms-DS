/// LeetCode #3810 - Minimum Operations to Reach Target Array
use std::collections::HashSet;

fn min_operations(nums: Vec<i32>, target: Vec<i32>) -> i32 {
    let s: HashSet<i32> = nums
        .iter()
        .zip(target.iter())
        .filter(|(x, y)| x != y)
        .map(|(x, _)| *x)
        .collect();
    s.len() as i32
}

fn main() {
    println!("{}", min_operations(vec![1, 2, 3], vec![2, 1, 3]));
}

#[cfg(test)]
mod tests {
    use super::min_operations;

    #[test]
    fn example1() {
        assert_eq!(min_operations(vec![1, 2, 3], vec![2, 1, 3]), 2);
    }

    #[test]
    fn example2() {
        assert_eq!(min_operations(vec![4, 1, 4], vec![5, 1, 4]), 1);
    }

    #[test]
    fn example3() {
        assert_eq!(min_operations(vec![7, 3, 7], vec![5, 5, 9]), 2);
    }
}
