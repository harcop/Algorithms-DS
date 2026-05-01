/// LeetCode #153 - Find Minimum in Rotated Sorted Array
fn find_min(nums: Vec<i32>) -> i32 {
    let mut lo = 0usize;
    let mut hi = nums.len() - 1;
    while lo < hi {
        let mid = lo + (hi - lo) / 2;
        if nums[mid] > nums[hi] {
            lo = mid + 1;
        } else {
            hi = mid;
        }
    }
    nums[lo]
}

fn main() {
    println!("{}", find_min(vec![3, 4, 5, 1, 2]));
}

#[cfg(test)]
mod tests {
    use super::find_min;

    #[test]
    fn example_one() {
        assert_eq!(find_min(vec![3, 4, 5, 1, 2]), 1);
    }

    #[test]
    fn example_two() {
        assert_eq!(find_min(vec![4, 5, 6, 7, 0, 1, 2]), 0);
    }

    #[test]
    fn example_three() {
        assert_eq!(find_min(vec![11, 13, 15, 17]), 11);
    }
}
