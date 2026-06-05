/// LeetCode #1746 - Maximum Subarray Sum After One Operation
fn max_sum_after_one_operation(nums: Vec<i32>) -> i32 {
    let mut res = nums[0];
    let mut cur = nums[0];
    let mut flip = i32::MIN / 4;
    for &x in nums.iter().skip(1) {
        flip = flip.max(cur - x);
        cur = cur.max(0) + x;
        res = res.max(cur).max(flip + x);
    }
    res
}
fn main() { println!("{}", max_sum_after_one_operation(vec![2, -1, -2, -3, 1, 3, -2, 3])); }
#[cfg(test)]
mod tests {
    use super::max_sum_after_one_operation;
    #[test]
    fn example_one() {
        assert_eq!(max_sum_after_one_operation(vec![2, -1, -2, -3, 1, 3, -2, 3]), 9);
    }
    #[test]
    fn example_two() {
        assert_eq!(max_sum_after_one_operation(vec![-1, -2, -3]), -1);
    }
}
