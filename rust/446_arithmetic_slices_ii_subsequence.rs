/// LeetCode #446 - Arithmetic Slices II - Subsequence
use std::collections::HashMap;

fn number_of_arithmetic_slices(nums: Vec<i32>) -> i32 {
    let n = nums.len();
    if n < 3 {
        return 0;
    }
    let mut dp: Vec<HashMap<i64, i32>> = vec![HashMap::new(); n];
    let mut ans = 0i32;
    for i in 0..n {
        for j in 0..i {
            let diff = nums[i] as i64 - nums[j] as i64;
            let mut sum = *dp[j].get(&diff).unwrap_or(&0);
            sum += 1;
            *dp[i].entry(diff).or_insert(0) += sum;
            ans += sum - 1;
        }
    }
    ans
}

fn main() {
    println!("{}", number_of_arithmetic_slices(vec![2, 4, 6, 8, 10]));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lc() {
        assert_eq!(number_of_arithmetic_slices(vec![2, 4, 6, 8, 10]), 7);
    }
}
