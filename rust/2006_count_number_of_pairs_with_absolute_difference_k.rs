/// LeetCode #2006 - Count Number of Pairs With Absolute Difference K
use std::collections::HashMap;

fn count_k_difference(nums: Vec<i32>, k: i32) -> i32 {
    if k == 0 {
        let mut cnt = HashMap::new();
        for x in nums {
            *cnt.entry(x).or_insert(0) += 1;
        }
        return cnt.values().map(|&c| c * (c - 1) / 2).sum();
    }
    let mut cnt = HashMap::new();
    let mut ans = 0i32;
    for x in nums {
        ans += cnt.get(&(x - k)).copied().unwrap_or(0);
        ans += cnt.get(&(x + k)).copied().unwrap_or(0);
        *cnt.entry(x).or_insert(0) += 1;
    }
    ans
}

fn main() {
    println!("{}", count_k_difference(vec![1, 2, 2, 1], 1));
}

#[cfg(test)]
mod tests {
    use super::count_k_difference;

    #[test]
    fn example_one() {
        assert_eq!(count_k_difference(vec![1, 2, 2, 1], 1), 4);
    }

    #[test]
    fn example_two() {
        assert_eq!(count_k_difference(vec![1, 3], 3), 0);
    }
}
