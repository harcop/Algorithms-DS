use std::collections::HashMap;

/// LeetCode #2845 - Count of Interesting Subarrays
fn count_interesting_subarrays(nums: Vec<i32>, modulo: i32, k: i32) -> i64 {
    let mut counts: HashMap<i32, i64> = HashMap::new();
    counts.insert(0, 1);
    let mut answer = 0;
    let mut sum = 0;

    for number in nums {
        if number % modulo == k {
            sum += 1;
        }
        let key = (sum - k).rem_euclid(modulo);
        answer += counts.get(&key).copied().unwrap_or(0);
        *counts.entry(sum % modulo).or_insert(0) += 1;
    }
    answer
}

fn main() {
    println!("{}", count_interesting_subarrays(vec![3, 2, 4], 2, 1));
}

#[cfg(test)]
mod tests {
    use super::count_interesting_subarrays;

    #[test]
    fn example_one() {
        assert_eq!(count_interesting_subarrays(vec![3, 2, 4], 2, 1), 3);
    }

    #[test]
    fn example_two() {
        assert_eq!(
            count_interesting_subarrays(vec![3, 1, 9, 6], 3, 0),
            2
        );
    }
}
