/// LeetCode #3105 - Longest Strictly Increasing or Strictly Decreasing Subarray
fn longest_monotonic_subarray(nums: Vec<i32>) -> i32 {
    let n = nums.len();
    let mut ans = 1;
    let mut t1 = 1;
    let mut t2 = 1;
    for i in 1..n {
        t1 = if nums[i] > nums[i - 1] { t1 + 1 } else { 1 };
        t2 = if nums[i] < nums[i - 1] { t2 + 1 } else { 1 };
        ans = ans.max(t1).max(t2);
    }
    ans
}

fn main() {
    println!("{}", longest_monotonic_subarray(vec![1, 4, 3, 3, 2]));
}

#[cfg(test)]
mod tests {
    use super::longest_monotonic_subarray;

    #[test]
    fn example1() {
        assert_eq!(longest_monotonic_subarray(vec![1, 4, 3, 3, 2]), 2);
    }

    #[test]
    fn example2() {
        assert_eq!(longest_monotonic_subarray(vec![3, 3, 3, 3]), 1);
    }

    #[test]
    fn example3() {
        assert_eq!(longest_monotonic_subarray(vec![3, 2, 1]), 3);
    }
}
