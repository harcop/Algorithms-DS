/// LeetCode #2488 - Count Subarrays With Median K
use std::collections::HashMap;

fn count_subarrays(nums: Vec<i32>, k: i32) -> i32 {
    let k_index = nums.iter().position(|&value| value == k).unwrap();
    let mut count = HashMap::new();
    let mut balance = 0;

    for i in (0..=k_index).rev() {
        if nums[i] < k {
            balance -= 1;
        } else if nums[i] > k {
            balance += 1;
        }
        *count.entry(balance).or_insert(0) += 1;
    }

    let mut answer = 0;
    balance = 0;
    for i in k_index..nums.len() {
        if nums[i] < k {
            balance -= 1;
        } else if nums[i] > k {
            balance += 1;
        }
        answer += count.get(&(-balance)).copied().unwrap_or(0);
        answer += count.get(&(1 - balance)).copied().unwrap_or(0);
    }

    answer
}

fn main() {
    println!("{}", count_subarrays(vec![3, 2, 1, 4, 5], 4));
}

#[cfg(test)]
mod tests {
    use super::count_subarrays;

    #[test]
    fn example_one() {
        assert_eq!(count_subarrays(vec![3, 2, 1, 4, 5], 4), 3);
    }

    #[test]
    fn example_two() {
        assert_eq!(count_subarrays(vec![2, 3, 1], 3), 1);
    }

    #[test]
    fn example_three() {
        assert_eq!(count_subarrays(vec![2, 5, 1, 4, 3, 6], 1), 3);
    }
}
