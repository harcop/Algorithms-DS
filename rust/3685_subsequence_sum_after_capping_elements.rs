/// LeetCode #3685 - Subsequence Sum After Capping Elements
fn subsequence_sum_after_capping(mut nums: Vec<i32>, k: i32) -> Vec<bool> {
    let n = nums.len();
    let k = k as usize;
    nums.sort_unstable();
    let mut result = vec![false; n];
    let mut dp = vec![false; k + 1];
    dp[0] = true;
    let mut i = 0usize;
    for x in 1..=n {
        while i < n && (nums[i] as usize) < x {
            let v = nums[i] as usize;
            for j in (v..=k).rev() {
                if dp[j - v] {
                    dp[j] = true;
                }
            }
            i += 1;
        }
        let remaining = n - i;
        let mut j = (k % x).max(k.saturating_sub(remaining * x));
        while j <= k {
            if dp[j] {
                result[x - 1] = true;
                break;
            }
            j += x;
        }
    }
    result
}

fn main() {
    println!("{:?}", subsequence_sum_after_capping(vec![4, 3, 2, 4], 5));
}

#[cfg(test)]
mod tests {
    use super::subsequence_sum_after_capping;

    #[test]
    fn example1() {
        assert_eq!(
            subsequence_sum_after_capping(vec![4, 3, 2, 4], 5),
            vec![false, false, true, true]
        );
    }

    #[test]
    fn example2() {
        assert_eq!(
            subsequence_sum_after_capping(vec![1, 2, 3, 4, 5], 3),
            vec![true, true, true, true, true]
        );
    }
}
