/// LeetCode #2393 - Count Strictly Increasing Subarrays
fn count_subarrays(nums: Vec<i32>) -> i64 {
    let mut ans = 1i64;
    let mut cnt = 1i64;
    for i in 1..nums.len() {
        if nums[i] > nums[i - 1] {
            cnt += 1;
        } else {
            cnt = 1;
        }
        ans += cnt;
    }
    ans
}

fn main() {
    println!("{}", count_subarrays(vec![1, 3, 5, 4, 4, 6]));
}

#[cfg(test)]
mod tests {
    use super::count_subarrays;

    #[test]
    fn example_one() {
        assert_eq!(count_subarrays(vec![1, 3, 5, 4, 4, 6]), 10);
    }

    #[test]
    fn example_two() {
        assert_eq!(count_subarrays(vec![1, 2, 3, 4, 5]), 15);
    }
}
