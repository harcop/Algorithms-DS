/// LeetCode #2915 - Length of the Longest Subsequence That Sums to Target
fn length_of_longest_subsequence(nums: Vec<i32>, target: i32) -> i32 {
    let target = target as usize;
    let mut dp = vec![i32::MIN / 2; target + 1];
    dp[0] = 0;
    for x in nums {
        let x = x as usize;
        for j in (x..=target).rev() {
            dp[j] = dp[j].max(dp[j - x] + 1);
        }
    }
    if dp[target] <= 0 {
        -1
    } else {
        dp[target]
    }
}

fn main() {
    println!("{}", length_of_longest_subsequence(vec![1, 2, 3, 4, 5], 9));
}

#[cfg(test)]
mod tests {
    use super::length_of_longest_subsequence;

    #[test]
    fn example_one() {
        assert_eq!(length_of_longest_subsequence(vec![1, 2, 3, 4, 5], 9), 3);
    }

    #[test]
    fn example_two() {
        assert_eq!(
            length_of_longest_subsequence(vec![4, 1, 3, 2, 1, 5], 7),
            4
        );
    }

    #[test]
    fn example_three() {
        assert_eq!(length_of_longest_subsequence(vec![1, 1, 5, 4, 5], 3), -1);
    }
}
