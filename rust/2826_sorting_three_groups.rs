/// LeetCode #2826 - Sorting Three Groups
fn minimum_operations(nums: Vec<i32>) -> i32 {
    let n = nums.len();
    let mut dp = vec![1; n];
    for i in 0..n {
        for j in 0..i {
            if nums[j] <= nums[i] {
                dp[i] = dp[i].max(dp[j] + 1);
            }
        }
    }
    (n - dp.into_iter().max().unwrap_or(0)) as i32
}

fn main() {
    println!("{}", minimum_operations(vec![2, 1, 3, 2, 1]));
}

#[cfg(test)]
mod tests {
    use super::minimum_operations;

    #[test]
    fn example_one() {
        assert_eq!(minimum_operations(vec![2, 1, 3, 2, 1]), 3);
    }

    #[test]
    fn example_two() {
        assert_eq!(minimum_operations(vec![1, 3, 2, 1, 3, 3]), 2);
    }

    #[test]
    fn example_three() {
        assert_eq!(minimum_operations(vec![2, 2, 2, 2, 3, 3]), 0);
    }
}
