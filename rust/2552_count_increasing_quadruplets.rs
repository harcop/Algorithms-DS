/// LeetCode #2552 - Count Increasing Quadruplets
fn count_quadruplets(nums: Vec<i32>) -> i64 {
    let n = nums.len();
    let mut dp = vec![0i64; n];
    let mut ans = 0i64;

    for k in 2..n {
        let mut num_less_than_k = 0;
        for j in 0..k {
            if nums[j] < nums[k] {
                num_less_than_k += 1;
                ans += dp[j];
            } else if nums[j] > nums[k] {
                dp[j] += num_less_than_k;
            }
        }
    }
    ans
}

fn main() {
    println!("{}", count_quadruplets(vec![1, 3, 2, 4, 5]));
}

#[cfg(test)]
mod tests {
    use super::count_quadruplets;

    #[test]
    fn example_one() {
        assert_eq!(count_quadruplets(vec![1, 3, 2, 4, 5]), 2);
    }

    #[test]
    fn example_two() {
        assert_eq!(count_quadruplets(vec![1, 2, 3, 4, 5]), 0);
    }
}
