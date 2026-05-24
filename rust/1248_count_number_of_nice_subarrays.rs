/// LeetCode #1248 - Count Number of Nice Subarrays
use std::collections::HashMap;

fn number_of_subarrays(nums: Vec<i32>, k: i32) -> i32 {
    let mut prefix = 0i32;
    let mut cnt: HashMap<i32, i32> = HashMap::new();
    cnt.insert(0, 1);
    let mut ans = 0i32;
    for &x in &nums {
        if x % 2 == 1 {
            prefix += 1;
        }
        if let Some(&c) = cnt.get(&(prefix - k)) {
            ans += c;
        }
        *cnt.entry(prefix).or_insert(0) += 1;
    }
    ans
}

fn main() {
    println!("{}", number_of_subarrays(vec![1, 1, 2, 1, 1], 3));
}

#[cfg(test)]
mod tests {
    use super::number_of_subarrays;

    #[test]
    fn example_one() {
        assert_eq!(number_of_subarrays(vec![1, 1, 2, 1, 1], 3), 2);
    }

    #[test]
    fn example_two() {
        assert_eq!(number_of_subarrays(vec![2, 4, 6], 1), 0);
    }

    #[test]
    fn example_three() {
        assert_eq!(
            number_of_subarrays(vec![2, 2, 2, 1, 2, 2, 1, 2, 2, 2], 2),
            16
        );
    }
}
