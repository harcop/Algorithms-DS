/// LeetCode #2875 - Minimum Size Subarray in Infinite Array
fn min_size_subarray(nums: Vec<i32>, target: i32) -> i32 {
    use std::collections::HashMap;

    let n = nums.len();
    let sum: i64 = nums.iter().map(|&x| x as i64).sum();
    let target = target as i64;
    let remaining = target % sum;
    let repeat_length = ((target / sum) * n as i64) as i32;
    if remaining == 0 {
        return repeat_length;
    }

    let mut best = n as i32;
    let mut prefix: i64 = 0;
    let mut prefix_to_index = HashMap::from([(0i64, -1i32)]);

    for i in 0..2 * n {
        prefix += nums[i % n] as i64;
        if let Some(&prev) = prefix_to_index.get(&(prefix - remaining)) {
            best = best.min(i as i32 - prev);
        }
        prefix_to_index.insert(prefix, i as i32);
    }

    if best == n as i32 {
        -1
    } else {
        repeat_length + best
    }
}

fn main() {
    println!("{}", min_size_subarray(vec![1, 2, 3], 5));
}

#[cfg(test)]
mod tests {
    use super::min_size_subarray;

    #[test]
    fn example_one() {
        assert_eq!(min_size_subarray(vec![1, 2, 3], 5), 2);
    }

    #[test]
    fn example_two() {
        assert_eq!(min_size_subarray(vec![1, 1, 1, 2, 3], 4), 2);
    }

    #[test]
    fn example_three() {
        assert_eq!(min_size_subarray(vec![2, 4, 6, 8], 3), -1);
    }
}
