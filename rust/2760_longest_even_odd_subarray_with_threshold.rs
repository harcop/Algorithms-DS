/// LeetCode #2760 - Longest Even Odd Subarray With Threshold
fn longest_alternating_subarray(nums: Vec<i32>, threshold: i32) -> i32 {
    let n = nums.len();
    let mut ans = 0;
    let mut l = 0;
    while l < n {
        if nums[l] % 2 == 0 && nums[l] <= threshold {
            let mut r = l + 1;
            while r < n && nums[r] % 2 != nums[r - 1] % 2 && nums[r] <= threshold {
                r += 1;
            }
            ans = ans.max((r - l) as i32);
            l = r;
        } else {
            l += 1;
        }
    }
    ans
}

fn main() {
    println!("{}", longest_alternating_subarray(vec![3, 2, 5, 4], 5));
}

#[cfg(test)]
mod tests {
    use super::longest_alternating_subarray;

    #[test]
    fn example_one() {
        assert_eq!(longest_alternating_subarray(vec![3, 2, 5, 4], 5), 3);
    }

    #[test]
    fn example_two() {
        assert_eq!(longest_alternating_subarray(vec![1, 2], 2), 1);
    }

    #[test]
    fn example_three() {
        assert_eq!(longest_alternating_subarray(vec![2, 3, 4, 5], 4), 3);
    }
}
