/// LeetCode #2461 - Maximum Sum of Distinct Subarrays With Length K
use std::collections::HashMap;

fn maximum_subarray_sum(nums: Vec<i32>, k: i32) -> i64 {
    let k = k as usize;
    let mut counts = HashMap::new();
    let mut sum = 0i64;
    let mut answer = 0i64;

    for (right, &num) in nums.iter().enumerate() {
        *counts.entry(num).or_insert(0) += 1;
        sum += num as i64;

        if right >= k {
            let removed = nums[right - k];
            sum -= removed as i64;
            let count = counts.get_mut(&removed).unwrap();
            *count -= 1;
            if *count == 0 {
                counts.remove(&removed);
            }
        }

        if right + 1 >= k && counts.len() == k {
            answer = answer.max(sum);
        }
    }

    answer
}

fn main() {
    println!("{}", maximum_subarray_sum(vec![1, 5, 4, 2, 9, 9, 9], 3));
}

#[cfg(test)]
mod tests {
    use super::maximum_subarray_sum;

    #[test]
    fn example_one() {
        assert_eq!(maximum_subarray_sum(vec![1, 5, 4, 2, 9, 9, 9], 3), 15);
    }

    #[test]
    fn no_distinct_window() {
        assert_eq!(maximum_subarray_sum(vec![4, 4, 4], 3), 0);
    }
}
