/// LeetCode #1537 - Get The Maximum Score
const MOD: i64 = 1_000_000_007;
fn max_sum(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
    let mut i = 0usize;
    let mut j = 0usize;
    let mut s1 = 0i64;
    let mut s2 = 0i64;
    let mut ans = 0i64;
    while i < nums1.len() && j < nums2.len() {
        if nums1[i] < nums2[j] { s1 = (s1 + nums1[i] as i64) % MOD; i += 1; }
        else if nums1[i] > nums2[j] { s2 = (s2 + nums2[j] as i64) % MOD; j += 1; }
        else {
            ans = (ans + s1.max(s2) + nums1[i] as i64) % MOD;
            s1 = 0; s2 = 0; i += 1; j += 1;
        }
    }
    while i < nums1.len() { s1 = (s1 + nums1[i] as i64) % MOD; i += 1; }
    while j < nums2.len() { s2 = (s2 + nums2[j] as i64) % MOD; j += 1; }
    (ans + s1.max(s2)) as i32 % MOD as i32
}
fn main() { println!("{}", max_sum(vec![2, 4, 5, 8, 10], vec![4, 6, 8, 9])); }
#[cfg(test)]
mod tests {
    use super::max_sum;
    #[test]
    fn example_one() { assert_eq!(max_sum(vec![2, 4, 5, 8, 10], vec![4, 6, 8, 9]), 30); }
    #[test]
    fn example_two() { assert_eq!(max_sum(vec![1, 4, 5], vec![2, 3, 4]), 14); }
}
