/// LeetCode #3729 - Count Distinct Subarrays Divisible by K in Sorted Array
use std::collections::HashMap;

fn num_good_subarrays(nums: Vec<i32>, k: i32) -> i64 {
    let k = k as i64;
    let mut cnt: HashMap<i64, i64> = HashMap::new();
    cnt.insert(0, 1);
    let mut ans = 0i64;
    let mut s = 0i64;
    for &x in &nums {
        s = (s + x as i64).rem_euclid(k);
        ans += cnt.get(&s).copied().unwrap_or(0);
        *cnt.entry(s).or_insert(0) += 1;
    }
    let n = nums.len();
    let mut i = 0;
    while i < n {
        let mut j = i + 1;
        while j < n && nums[j] == nums[i] {
            j += 1;
        }
        let m = (j - i) as i64;
        for h in 1..=m {
            if nums[i] as i64 * h % k == 0 {
                ans -= m - h;
            }
        }
        i = j;
    }
    ans
}

fn main() {
    println!("{}", num_good_subarrays(vec![1, 2, 3], 3));
}

#[cfg(test)]
mod tests {
    use super::num_good_subarrays;

    #[test]
    fn example1() {
        assert_eq!(num_good_subarrays(vec![1, 2, 3], 3), 3);
    }

    #[test]
    fn example2() {
        assert_eq!(num_good_subarrays(vec![2, 2, 2, 2, 2, 2], 6), 2);
    }
}
