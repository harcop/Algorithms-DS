/// LeetCode #2537 - Count the Number of Good Subarrays
use std::collections::HashMap;

fn count_good(nums: Vec<i32>, k: i32) -> i64 {
    let mut cnt: HashMap<i32, i64> = HashMap::new();
    let mut ans = 0i64;
    let mut pairs = 0i64;
    let mut l = 0usize;
    for r in 0..nums.len() {
        pairs += *cnt.get(&nums[r]).unwrap_or(&0);
        *cnt.entry(nums[r]).or_default() += 1;
        while pairs >= k as i64 {
            *cnt.get_mut(&nums[l]).unwrap() -= 1;
            pairs -= cnt[&nums[l]];
            l += 1;
        }
        ans += l as i64;
    }
    ans
}

fn main() {
    println!("{}", count_good(vec![1, 1, 1, 1, 1], 10));
}

#[cfg(test)]
mod tests {
    use super::count_good;

    #[test]
    fn example_one() {
        assert_eq!(count_good(vec![1, 1, 1, 1, 1], 10), 1);
    }

    #[test]
    fn example_two() {
        assert_eq!(count_good(vec![3, 1, 4, 3, 2, 2, 4], 2), 4);
    }
}
