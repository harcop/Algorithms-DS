/// LeetCode #1060 - Missing Element in Sorted Array
fn missing_element(nums: Vec<i32>, k: i32) -> i32 {
    let n = nums.len();
    let mut lo = 0usize;
    let mut hi = n - 1;
    while lo < hi {
        let mid = lo + (hi - lo) / 2;
        let missing = nums[mid] as i64 - nums[0] as i64 - mid as i64;
        if missing < k as i64 {
            lo = mid + 1;
        } else {
            hi = mid;
        }
    }
    if lo == 0 {
        return nums[0] + k - 1;
    }
    let missing_before = nums[lo - 1] as i64 - nums[0] as i64 - (lo - 1) as i64;
    (nums[lo - 1] as i64 + (k as i64 - missing_before)) as i32
}

fn main() {
    println!("{}", missing_element(vec![4, 7, 9, 10], 3));
}

#[cfg(test)]
mod tests {
    use super::missing_element;

    #[test]
    fn example_one() {
        assert_eq!(missing_element(vec![4, 7, 9, 10], 3), 8);
    }

    #[test]
    fn example_two() {
        assert_eq!(missing_element(vec![4, 7, 9, 10], 1), 5);
    }
}
