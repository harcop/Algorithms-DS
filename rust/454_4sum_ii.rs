/// LeetCode #454 - 4Sum II
use std::collections::HashMap;

fn four_sum_count(nums1: Vec<i32>, nums2: Vec<i32>, nums3: Vec<i32>, nums4: Vec<i32>) -> i32 {
    let mut m = HashMap::new();
    for a in &nums1 {
        for b in &nums2 {
            *m.entry(a + b).or_insert(0) += 1;
        }
    }
    let mut ans = 0;
    for c in &nums3 {
        for d in &nums4 {
            let k = -(c + d);
            ans += *m.get(&k).unwrap_or(&0);
        }
    }
    ans
}

fn main() {
    println!(
        "{}",
        four_sum_count(vec![1, 2], vec![-2, -1], vec![-1, 2], vec![0, 2])
    );
}

#[cfg(test)]
mod tests {
    use super::four_sum_count;

    #[test]
    fn example_one() {
        assert_eq!(
            four_sum_count(vec![1, 2], vec![-2, -1], vec![-1, 2], vec![0, 2]),
            2
        );
    }
}
