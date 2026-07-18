/// LeetCode #2464 - Minimum Subarrays in a Valid Split
fn valid_subarray_split(nums: Vec<i32>) -> i32 {
    fn gcd(mut a: i32, mut b: i32) -> i32 {
        while b != 0 {
            let temp = a % b;
            a = b;
            b = temp;
        }
        a
    }

    let n = nums.len();
    let mut dp = vec![i32::MAX / 2; n];

    for i in 0..n {
        for j in 0..=i {
            if gcd(nums[j], nums[i]) > 1 {
                let previous = if j == 0 { 0 } else { dp[j - 1] };
                dp[i] = dp[i].min(previous + 1);
            }
        }
    }

    if dp[n - 1] >= i32::MAX / 2 {
        -1
    } else {
        dp[n - 1]
    }
}

fn main() {
    println!("{}", valid_subarray_split(vec![2, 6, 3, 4, 3]));
}

#[cfg(test)]
mod tests {
    use super::valid_subarray_split;

    #[test]
    fn example_one() {
        assert_eq!(valid_subarray_split(vec![2, 6, 3, 4, 3]), 2);
    }

    #[test]
    fn example_two() {
        assert_eq!(valid_subarray_split(vec![3, 5]), 2);
    }

    #[test]
    fn example_three() {
        assert_eq!(valid_subarray_split(vec![1, 2, 1]), -1);
    }
}
