/// LeetCode #1438 - Longest Continuous Subarray With Absolute Diff Less Than Or Equal To Limit
use std::collections::BTreeMap;

fn longest_subarray(nums: Vec<i32>, limit: i32) -> i32 {
    let mut left = 0usize;
    let mut freq = BTreeMap::new();
    let mut best = 0i32;
    for right in 0..nums.len() {
        *freq.entry(nums[right]).or_insert(0i32) += 1;
        while freq.last_key_value().unwrap().0 - freq.first_key_value().unwrap().0 > limit {
            let v = nums[left];
            let e = freq.get_mut(&v).unwrap();
            *e -= 1;
            if *e == 0 {
                freq.remove(&v);
            }
            left += 1;
        }
        best = best.max((right - left + 1) as i32);
    }
    best
}

fn main() {
    println!("{}", longest_subarray(vec![8, 2, 4, 7], 4));
}

#[cfg(test)]
mod tests {
    use super::longest_subarray;

    #[test]
    fn example_one() {
        assert_eq!(longest_subarray(vec![8, 2, 4, 7], 4), 2);
    }

    #[test]
    fn example_two() {
        assert_eq!(longest_subarray(vec![10, 1, 2, 4, 7, 2], 5), 4);
    }
}

