/// LeetCode #2774 - Array Upper Bound (JS problem; Rust analogue)
/// Return the last index of `target` in a sorted ascending array, or -1.
fn upper_bound(nums: &[i32], target: i32) -> i32 {
    let mut left = 0usize;
    let mut right = nums.len();
    while left < right {
        let mid = left + (right - left) / 2;
        if nums[mid] > target {
            right = mid;
        } else {
            left = mid + 1;
        }
    }
    if left > 0 && nums[left - 1] == target {
        (left - 1) as i32
    } else {
        -1
    }
}

fn main() {
    println!("{}", upper_bound(&[3, 4, 6, 6, 6, 6, 7], 6));
}

#[cfg(test)]
mod tests {
    use super::upper_bound;

    #[test]
    fn example_one() {
        assert_eq!(upper_bound(&[3, 4, 5], 5), 2);
    }

    #[test]
    fn example_two() {
        assert_eq!(upper_bound(&[1, 4, 5], 2), -1);
    }

    #[test]
    fn example_three() {
        assert_eq!(upper_bound(&[3, 4, 6, 6, 6, 6, 7], 6), 5);
    }
}
