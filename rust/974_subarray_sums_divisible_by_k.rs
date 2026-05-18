/// LeetCode #974 - Subarray Sums Divisible by K
use std::collections::HashMap;

fn subarrays_div_by_k(nums: Vec<i32>, k: i32) -> i32 {
    let mut freq: HashMap<i32, i32> = HashMap::new();
    freq.insert(0, 1);
    let mut sum = 0i32;
    let mut ans = 0i32;
    for x in nums {
        sum += x;
        let rem = ((sum % k) + k) % k;
        ans += *freq.get(&rem).unwrap_or(&0);
        *freq.entry(rem).or_insert(0) += 1;
    }
    ans
}

fn main() {
    println!("{}", subarrays_div_by_k(vec![4, 5, 0, -2, -3, 1], 5));
}

#[cfg(test)]
mod tests {
    use super::subarrays_div_by_k;

    #[test]
    fn example_one() {
        assert_eq!(subarrays_div_by_k(vec![4, 5, 0, -2, -3, 1], 5), 7);
    }

    #[test]
    fn example_two() {
        assert_eq!(subarrays_div_by_k(vec![5], 9), 0);
    }
}
