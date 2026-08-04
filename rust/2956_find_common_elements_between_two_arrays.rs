/// LeetCode #2956 - Find Common Elements Between Two Arrays
use std::collections::HashSet;

fn find_intersection_values(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
    let s1: HashSet<_> = nums1.iter().copied().collect();
    let s2: HashSet<_> = nums2.iter().copied().collect();
    let a = nums1.iter().filter(|x| s2.contains(x)).count() as i32;
    let b = nums2.iter().filter(|x| s1.contains(x)).count() as i32;
    vec![a, b]
}

fn main() {
    println!("{:?}", find_intersection_values(vec![2, 3, 2], vec![1, 2]));
}

#[cfg(test)]
mod tests {
    use super::find_intersection_values;

    #[test]
    fn example_one() {
        assert_eq!(find_intersection_values(vec![2, 3, 2], vec![1, 2]), vec![2, 1]);
    }

    #[test]
    fn example_two() {
        assert_eq!(
            find_intersection_values(vec![4, 3, 2, 3, 1], vec![2, 2, 5, 2, 3, 6]),
            vec![3, 4]
        );
    }

    #[test]
    fn example_three() {
        assert_eq!(find_intersection_values(vec![3, 4, 2, 3], vec![1, 5]), vec![0, 0]);
    }
}
