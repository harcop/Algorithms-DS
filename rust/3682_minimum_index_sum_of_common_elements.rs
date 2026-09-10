/// LeetCode #3682 - Minimum Index Sum of Common Elements (premium)
use std::collections::HashMap;

fn minimum_sum(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
    let mut d = HashMap::new();
    for (i, &x) in nums2.iter().enumerate() {
        d.entry(x).or_insert(i);
    }
    let mut ans = i32::MAX;
    for (i, &x) in nums1.iter().enumerate() {
        if let Some(&j) = d.get(&x) {
            ans = ans.min((i + j) as i32);
        }
    }
    if ans == i32::MAX {
        -1
    } else {
        ans
    }
}

fn main() {
    println!("{}", minimum_sum(vec![3, 2, 1], vec![1, 3, 1]));
}

#[cfg(test)]
mod tests {
    use super::minimum_sum;

    #[test]
    fn example1() {
        assert_eq!(minimum_sum(vec![3, 2, 1], vec![1, 3, 1]), 1);
    }

    #[test]
    fn example2() {
        assert_eq!(minimum_sum(vec![5, 1, 2], vec![2, 1, 3]), 2);
    }

    #[test]
    fn example3() {
        assert_eq!(minimum_sum(vec![6, 4], vec![7, 8]), -1);
    }
}
