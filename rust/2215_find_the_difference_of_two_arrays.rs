/// LeetCode #2215 - Find the Difference of Two Arrays
use std::collections::HashSet;

fn find_difference(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<Vec<i32>> {
    let set1: HashSet<i32> = nums1.into_iter().collect();
    let set2: HashSet<i32> = nums2.into_iter().collect();

    let only1: Vec<i32> = set1.iter().filter(|x| !set2.contains(x)).copied().collect();
    let only2: Vec<i32> = set2.iter().filter(|x| !set1.contains(x)).copied().collect();

    vec![only1, only2]
}

fn main() {
    println!("{:?}", find_difference(vec![1, 2, 3], vec![2, 4, 6]));
}

#[cfg(test)]
mod tests {
    use super::find_difference;

    fn sort(mut v: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        for inner in &mut v {
            inner.sort();
        }
        v
    }

    #[test]
    fn example_one() {
        assert_eq!(
            sort(find_difference(vec![1, 2, 3], vec![2, 4, 6])),
            vec![vec![1, 3], vec![4, 6]]
        );
    }

    #[test]
    fn example_two() {
        assert_eq!(
            sort(find_difference(vec![1, 2, 3, 3], vec![1, 1, 2, 2])),
            vec![vec![3], vec![]]
        );
    }
}
