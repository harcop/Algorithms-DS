/// LeetCode #560 - Subarray Sum Equals K
use std::collections::HashMap;

fn subarray_sum(nums: Vec<i32>, k: i32) -> i32 {
    let mut pref = HashMap::new();
    pref.insert(0, 1);
    let mut sum = 0i32;
    let mut ans = 0i32;
    for x in nums {
        sum += x;
        ans += *pref.get(&(sum - k)).unwrap_or(&0);
        *pref.entry(sum).or_insert(0) += 1;
    }
    ans
}

fn main() {
    println!("{}", subarray_sum(vec![1, 1, 1], 2));
}

#[cfg(test)]
mod tests {
    use super::subarray_sum;

    #[test]
    fn example_one() {
        assert_eq!(subarray_sum(vec![1, 1, 1], 2), 2);
    }
}
