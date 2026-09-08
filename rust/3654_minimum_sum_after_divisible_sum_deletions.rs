/// LeetCode #3654 - Minimum Sum After Divisible Sum Deletions
fn min_array_sum(nums: Vec<i32>, k: i32) -> i64 {
    let k = k as usize;
    let mut dp = vec![i64::MAX; k];
    dp[0] = 0;
    let mut result = 0i64;
    for x in nums {
        result += x as i64;
        let r = (result % k as i64) as usize;
        result = result.min(dp[r]);
        dp[r] = result;
    }
    result
}

fn main() {
    println!("{}", min_array_sum(vec![1, 1, 1], 2));
}

#[cfg(test)]
mod tests {
    use super::min_array_sum;

    #[test]
    fn example1() {
        assert_eq!(min_array_sum(vec![1, 1, 1], 2), 1);
    }

    #[test]
    fn example2() {
        assert_eq!(min_array_sum(vec![3, 1, 4, 1, 5], 3), 5);
    }
}
