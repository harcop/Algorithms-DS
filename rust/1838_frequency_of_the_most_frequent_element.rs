/// LeetCode #1838 - Frequency of the Most Frequent Element
fn max_frequency(mut nums: Vec<i32>, k: i32) -> i32 {
    nums.sort_unstable();
    let mut left = 0usize;
    let mut window_sum = 0i64;
    let mut best = 0usize;

    for right in 0..nums.len() {
        window_sum += nums[right] as i64;
        while nums[right] as i64 * (right - left + 1) as i64 - window_sum > k as i64 {
            window_sum -= nums[left] as i64;
            left += 1;
        }
        best = best.max(right - left + 1);
    }
    best as i32
}

fn main() {
    println!("{}", max_frequency(vec![1, 2, 4], 5));
}

#[cfg(test)]
mod tests {
    use super::max_frequency;

    #[test]
    fn example_one() {
        assert_eq!(max_frequency(vec![1, 2, 4], 5), 3);
    }

    #[test]
    fn example_two() {
        assert_eq!(max_frequency(vec![1, 4, 8, 13], 5), 2);
    }

    #[test]
    fn example_three() {
        assert_eq!(max_frequency(vec![3, 9, 6], 2), 1);
    }
}
