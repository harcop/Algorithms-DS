/// LeetCode #1035 - Uncrossed Lines
fn max_uncrossed_lines(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
    let n = nums1.len();
    let m = nums2.len();
    let mut dp = vec![vec![0i32; m + 1]; n + 1];
    for i in 1..=n {
        for j in 1..=m {
            if nums1[i - 1] == nums2[j - 1] {
                dp[i][j] = dp[i - 1][j - 1] + 1;
            } else {
                dp[i][j] = dp[i - 1][j].max(dp[i][j - 1]);
            }
        }
    }
    dp[n][m]
}

fn main() {
    println!("{}", max_uncrossed_lines(vec![1, 4, 2], vec![1, 2, 4]));
}

#[cfg(test)]
mod tests {
    use super::max_uncrossed_lines;

    #[test]
    fn example_one() {
        assert_eq!(max_uncrossed_lines(vec![1, 4, 2], vec![1, 2, 4]), 2);
    }

    #[test]
    fn example_two() {
        assert_eq!(max_uncrossed_lines(vec![2, 5, 1, 2, 5], vec![10, 5, 2, 1, 5, 2]), 3);
    }
}
