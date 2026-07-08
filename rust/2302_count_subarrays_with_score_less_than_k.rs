/// LeetCode #2302 - Count Subarrays With Score Less Than K
fn count_subarrays(nums: Vec<i32>, k: i64) -> i64 {
    let mut ans = 0i64;
    let mut sum = 0i64;
    let mut j = 0usize;
    for i in 0..nums.len() {
        sum += nums[i] as i64;
        while sum * ((i - j + 1) as i64) >= k {
            sum -= nums[j] as i64;
            j += 1;
        }
        ans += (i - j + 1) as i64;
    }
    ans
}

fn main() {
    println!("{}", count_subarrays(vec![2, 1, 4, 3, 5], 10));
}

#[cfg(test)]
mod tests {
    use super::count_subarrays;

    #[test]
    fn example_one() {
        assert_eq!(count_subarrays(vec![2, 1, 4, 3, 5], 10), 6);
    }

    #[test]
    fn example_two() {
        assert_eq!(count_subarrays(vec![1, 1, 1], 5), 5);
    }
}
