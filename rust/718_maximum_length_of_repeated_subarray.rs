/// LeetCode #718 - Maximum Length of Repeated Subarray
fn find_length(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
    let m = nums1.len(); let n = nums2.len();
    let mut dp = vec![vec![0i32; n + 1]; m + 1];
    let mut best = 0i32;
    for i in 1..=m {
        for j in 1..=n {
            if nums1[i - 1] == nums2[j - 1] {
                dp[i][j] = dp[i - 1][j - 1] + 1;
                best = best.max(dp[i][j]);
            }
        }
    }
    best
}

fn main() {
    println!("{}", find_length(vec![1,2,3,2,1], vec![3,2,1,4,7]));
}

#[cfg(test)]
mod tests {
    use super::find_length;

    #[test]
    fn example_one() {
        assert_eq!(find_length(vec![1,2,3,2,1], vec![3,2,1,4,7]), 3);
    }
}
