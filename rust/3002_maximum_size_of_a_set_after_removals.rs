/// LeetCode #3002 - Maximum Size of a Set After Removals
use std::collections::HashSet;

fn maximum_set_size(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
    let n = nums1.len();
    let s1: HashSet<i32> = nums1.into_iter().collect();
    let s2: HashSet<i32> = nums2.into_iter().collect();
    let half = n / 2;
    let only1 = s1.difference(&s2).count();
    let only2 = s2.difference(&s1).count();
    let common = s1.intersection(&s2).count();
    let a = only1.min(half);
    let b = only2.min(half);
    (a + b + common).min(n) as i32
}

fn main() {
    println!(
        "{}",
        maximum_set_size(vec![1, 2, 1, 2], vec![1, 1, 1, 1])
    );
}

#[cfg(test)]
mod tests {
    use super::maximum_set_size;

    #[test]
    fn example_one() {
        assert_eq!(maximum_set_size(vec![1, 2, 1, 2], vec![1, 1, 1, 1]), 2);
    }

    #[test]
    fn example_two() {
        assert_eq!(
            maximum_set_size(vec![1, 2, 3, 4, 5, 6], vec![2, 3, 2, 3, 2, 3]),
            5
        );
    }

    #[test]
    fn example_three() {
        assert_eq!(
            maximum_set_size(vec![1, 1, 2, 2, 3, 3], vec![4, 4, 5, 5, 6, 6]),
            6
        );
    }
}
