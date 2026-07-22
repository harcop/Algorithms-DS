/// LeetCode #2588 - Count the Number of Beautiful Subarrays
use std::collections::HashMap;

fn beautiful_subarrays(nums: Vec<i32>) -> i64 {
    let mut cnt: HashMap<i32, i64> = HashMap::new();
    cnt.insert(0, 1);
    let mut ans = 0i64;
    let mut mask = 0;
    for x in nums {
        mask ^= x;
        ans += *cnt.get(&mask).unwrap_or(&0);
        *cnt.entry(mask).or_insert(0) += 1;
    }
    ans
}

fn main() {
    println!("{}", beautiful_subarrays(vec![4, 3, 1, 2, 4]));
}

#[cfg(test)]
mod tests {
    use super::beautiful_subarrays;

    #[test]
    fn example_one() {
        assert_eq!(beautiful_subarrays(vec![4, 3, 1, 2, 4]), 2);
    }

    #[test]
    fn example_two() {
        assert_eq!(beautiful_subarrays(vec![1, 10, 4]), 0);
    }
}
