/// LeetCode #81 - Search in Rotated Sorted Array II
fn search(nums: Vec<i32>, target: i32) -> bool {
    let mut lo = 0usize;
    let mut hi = nums.len();

    while lo < hi {
        let mid = lo + (hi - lo) / 2;
        if nums[mid] == target {
            return true;
        }
        if nums[lo] == nums[mid] && nums[mid] == nums[hi.saturating_sub(1)] {
            lo += 1;
            continue;
        }
        if nums[lo] <= nums[mid] {
            if nums[lo] <= target && target < nums[mid] {
                hi = mid;
            } else {
                lo = mid + 1;
            }
        } else if nums[mid] < target && target <= nums[hi.saturating_sub(1)] {
            lo = mid + 1;
        } else {
            hi = mid;
        }
    }
    false
}

fn main() {
    println!("{}", search(vec![2, 5, 6, 0, 0, 1, 2], 0));
}

#[cfg(test)]
mod tests {
    use super::search;

    #[test]
    fn example_one() {
        assert!(search(vec![2, 5, 6, 0, 0, 1, 2], 0));
    }

    #[test]
    fn example_two() {
        assert!(!search(vec![2, 5, 6, 0, 0, 1, 2], 3));
    }

    #[test]
    fn example_three() {
        assert!(search(vec![1, 0, 1, 1, 1], 0));
    }
}
