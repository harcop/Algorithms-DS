/// LeetCode #1458 - Max Dot Product Of Two Subsequences
fn max_dot_product(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
    let n = nums1.len();
    let m = nums2.len();
    let mut dp = vec![0i32; m + 1];
    let mut ans = i32::MIN;
    for i in 0..n {
        let mut prev = 0;
        for j in 0..m {
            let tmp = dp[j + 1];
            dp[j + 1] = (nums1[i] * nums2[j] + prev).max(dp[j + 1]).max(0);
            prev = tmp;
            ans = ans.max(dp[j + 1]);
        }
    }
    ans.max(0)
}
fn main() { println!("{}", max_dot_product(vec![2,1,-2,5], vec![3,0,-6])); }
#[cfg(test)]
mod tests {
    use super::max_dot_product;
    #[test]
    fn example_one() { assert_eq!(max_dot_product(vec![2,1,-2,5], vec![3,0,-6]), 18); }
    #[test]
    fn example_two() { assert_eq!(max_dot_product(vec![3,-2], vec![2,-6,7]), 21); }
    #[test]
    fn example_three() { assert_eq!(max_dot_product(vec![-1,-1], vec![1,1]), 0); }
}