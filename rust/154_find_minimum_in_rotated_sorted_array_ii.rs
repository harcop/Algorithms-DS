/// LeetCode #154 - Find Minimum in Rotated Sorted Array II
fn find_min(nums: Vec<i32>) -> i32 {
    let mut lo = 0usize;
    let mut hi = nums.len() - 1;
    while lo < hi {
        let mid = lo + (hi - lo) / 2;
        if nums[mid] > nums[hi] {
            lo = mid + 1;
        } else if nums[mid] < nums[hi] {
            hi = mid;
        } else {
            hi -= 1;
        }
    }
    nums[lo]
}

fn main() {
    println!("{}", find_min(vec![2, 2, 2, 0, 1]));
}

#[cfg(test)]
mod tests {
    use super::find_min;

    #[test]
    fn example_one() {
        assert_eq!(find_min(vec![1, 3, 5]), 1);
    }

    #[test]
    fn example_two() {
        assert_eq!(find_min(vec![2, 2, 2, 0, 1]), 0);
    }
}
