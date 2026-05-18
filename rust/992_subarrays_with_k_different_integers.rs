/// LeetCode #992 - Subarrays with K Different Integers
fn subarrays_with_k_distinct(nums: Vec<i32>, k: i32) -> i32 {
    fn at_most(nums: &[i32], k: i32) -> i32 {
        use std::collections::HashMap;
        let mut freq = HashMap::new();
        let mut left = 0usize;
        let mut res = 0i32;
        for right in 0..nums.len() {
            *freq.entry(nums[right]).or_insert(0i32) += 1;
            while freq.len() > k as usize {
                let v = freq.get_mut(&nums[left]).unwrap();
                *v -= 1;
                if *v == 0 { freq.remove(&nums[left]); }
                left += 1;
            }
            res += (right - left + 1) as i32;
        }
        res
    }
    at_most(&nums, k) - at_most(&nums, k - 1)
}

fn main() {
    println!("{}", subarrays_with_k_distinct(vec![1, 2, 1, 2, 3], 2));
}

#[cfg(test)]
mod tests {
    use super::subarrays_with_k_distinct;

    #[test]
    fn example_one() {
        assert_eq!(subarrays_with_k_distinct(vec![1, 2, 1, 2, 3], 2), 7);
    }

    #[test]
    fn example_two() {
        assert_eq!(subarrays_with_k_distinct(vec![2, 1, 3], 1), 3);
    }
}
