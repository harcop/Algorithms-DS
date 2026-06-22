/// LeetCode #2032 - Two Out of Three
use std::collections::HashSet;

fn two_out_of_three(nums1: Vec<i32>, nums2: Vec<i32>, nums3: Vec<i32>) -> Vec<i32> {
    let s1: HashSet<i32> = nums1.into_iter().collect();
    let s2: HashSet<i32> = nums2.into_iter().collect();
    let s3: HashSet<i32> = nums3.into_iter().collect();
    (1..=100)
        .filter(|&i| {
            (s1.contains(&i) as i32 + s2.contains(&i) as i32 + s3.contains(&i) as i32) > 1
        })
        .collect()
}

fn main() {
    println!(
        "{:?}",
        two_out_of_three(vec![1, 1, 3, 2], vec![2, 3], vec![3])
    );
}

#[cfg(test)]
mod tests {
    use super::two_out_of_three;

    fn sorted(v: Vec<i32>) -> Vec<i32> {
        let mut v = v;
        v.sort_unstable();
        v
    }

    #[test]
    fn example_one() {
        assert_eq!(
            sorted(two_out_of_three(vec![1, 1, 3, 2], vec![2, 3], vec![3])),
            vec![2, 3]
        );
    }

    #[test]
    fn example_two() {
        assert_eq!(
            sorted(two_out_of_three(vec![3, 1], vec![2, 3], vec![1, 2])),
            vec![1, 2, 3]
        );
    }
}
