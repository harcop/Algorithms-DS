/// LeetCode #209 - Minimum Size Subarray Sum
fn min_sub_array_len(target: i32, nums: Vec<i32>) -> i32 {
    let mut lo = 0usize;
    let mut sum = 0i32;
    let mut best = usize::MAX;
    for hi in 0..nums.len() {
        sum += nums[hi];
        while sum >= target {
            best = best.min(hi - lo + 1);
            sum -= nums[lo];
            lo += 1;
        }
    }
    if best == usize::MAX {
        0
    } else {
        best as i32
    }
}

fn main() {
    println!("{}", min_sub_array_len(7, vec![2, 3, 1, 2, 4, 3]));
}

#[cfg(test)]
mod tests {
    use super::min_sub_array_len;

    #[test]
    fn example_one() {
        assert_eq!(min_sub_array_len(7, vec![2, 3, 1, 2, 4, 3]), 2);
    }

    #[test]
    fn example_two() {
        assert_eq!(min_sub_array_len(4, vec![1, 4, 4]), 1);
    }

    #[test]
    fn example_three() {
        assert_eq!(min_sub_array_len(11, vec![1, 1, 1, 1, 1, 1, 1, 1]), 0);
    }
}
