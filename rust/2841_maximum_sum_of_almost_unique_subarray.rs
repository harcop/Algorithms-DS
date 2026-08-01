use std::collections::HashMap;

/// LeetCode #2841 - Maximum Sum of Almost Unique Subarray
fn max_sum(nums: Vec<i32>, m: i32, k: i32) -> i64 {
    let k = k as usize;
    let mut counts = HashMap::<i32, i32>::new();
    let mut window_sum = 0i64;
    let mut answer = 0i64;

    for (right, &value) in nums.iter().enumerate() {
        window_sum += value as i64;
        *counts.entry(value).or_default() += 1;

        if right >= k {
            let outgoing = nums[right - k];
            window_sum -= outgoing as i64;
            let count = counts.get_mut(&outgoing).unwrap();
            *count -= 1;
            if *count == 0 {
                counts.remove(&outgoing);
            }
        }

        if right + 1 >= k && counts.len() >= m as usize {
            answer = answer.max(window_sum);
        }
    }
    answer
}

fn main() {
    println!("{}", max_sum(vec![2, 6, 7, 3, 1, 7], 3, 4));
}

#[cfg(test)]
mod tests {
    use super::max_sum;

    #[test]
    fn examples() {
        assert_eq!(max_sum(vec![2, 6, 7, 3, 1, 7], 3, 4), 18);
        assert_eq!(max_sum(vec![5, 9, 9, 2, 4, 5, 4], 1, 3), 23);
        assert_eq!(max_sum(vec![1, 2, 1, 2, 1, 2, 1], 3, 3), 0);
    }
}
