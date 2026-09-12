/// LeetCode #3712 - Sum of Elements With Frequency Divisible by K
use std::collections::HashMap;

fn sum_divisible_by_k(nums: Vec<i32>, k: i32) -> i32 {
    let mut cnt: HashMap<i32, i32> = HashMap::new();
    for &x in &nums {
        *cnt.entry(x).or_insert(0) += 1;
    }
    nums.into_iter()
        .filter(|x| cnt[x] % k == 0)
        .sum()
}

fn main() {
    println!("{}", sum_divisible_by_k(vec![1, 2, 2, 3, 3, 3, 3, 4], 2));
}

#[cfg(test)]
mod tests {
    use super::sum_divisible_by_k;

    #[test]
    fn example1() {
        assert_eq!(sum_divisible_by_k(vec![1, 2, 2, 3, 3, 3, 3, 4], 2), 16);
    }

    #[test]
    fn example2() {
        assert_eq!(sum_divisible_by_k(vec![1, 2, 3, 4, 5], 2), 0);
    }

    #[test]
    fn example3() {
        assert_eq!(sum_divisible_by_k(vec![4, 4, 4, 1, 2, 3], 3), 12);
    }
}
