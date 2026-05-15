/// LeetCode #795 - Number of Subarrays with Bounded Maximum
fn num_subarray_bounded_max(nums: Vec<i32>, left: i32, right: i32) -> i32 {
    fn at_most(nums: &[i32], bound: i32) -> i32 {
        let mut ans = 0i32;
        let mut j = 0usize;
        for i in 0..nums.len() {
            if nums[i] > bound {
                j = i + 1;
            } else {
                ans += (i - j + 1) as i32;
            }
        }
        ans
    }
    at_most(&nums, right) - at_most(&nums, left - 1)
}

fn main() {
    println!("{}", num_subarray_bounded_max(vec![2, 1, 4, 3], 2, 3));
}

#[cfg(test)]
mod tests {
    use super::num_subarray_bounded_max;

    #[test]
    fn example_one() {
        assert_eq!(num_subarray_bounded_max(vec![2, 1, 4, 3], 2, 3), 3);
    }
}
