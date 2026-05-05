/// LeetCode #349 - Intersection of Two Arrays
use std::collections::HashSet;

fn intersection(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
    let s1: HashSet<i32> = nums1.into_iter().collect();
    let s2: HashSet<i32> = nums2.into_iter().collect();
    s1.intersection(&s2).copied().collect()
}

fn main() {
    println!("{:?}", intersection(vec![1,2,2,1], vec![2,2]));
}

#[cfg(test)]
mod tests {
    use super::intersection;

    #[test]
    fn example_one() {
        let mut v = intersection(vec![1,2,2,1], vec![2,2]);
        v.sort_unstable();
        assert_eq!(v, vec![2]);
    }
}
