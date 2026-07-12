/// LeetCode #2364 - Count Number of Bad Pairs
use std::collections::HashMap;

fn count_bad_pairs(nums: Vec<i32>) -> i64 {
    let mut ans = 0i64;
    let mut count: HashMap<i32, i64> = HashMap::new();

    for (i, &num) in nums.iter().enumerate() {
        let key = num - i as i32;
        let good = *count.get(&key).unwrap_or(&0);
        ans += i as i64 - good;
        *count.entry(key).or_insert(0) += 1;
    }

    ans
}

fn main() {
    println!("{}", count_bad_pairs(vec![4, 1, 3, 3]));
}

#[cfg(test)]
mod tests {
    use super::count_bad_pairs;

    #[test]
    fn example_one() {
        assert_eq!(count_bad_pairs(vec![4, 1, 3, 3]), 5);
    }

    #[test]
    fn example_two() {
        assert_eq!(count_bad_pairs(vec![1, 2, 3, 4, 5]), 0);
    }
}
