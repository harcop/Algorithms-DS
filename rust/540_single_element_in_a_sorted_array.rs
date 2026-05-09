/// LeetCode #540 - Single Element in a Sorted Array
fn single_non_duplicate(nums: Vec<i32>) -> i32 {
    let (mut lo, mut hi) = (0usize, nums.len() - 1);
    while lo < hi {
        let mid = lo + (hi - lo) / 2;
        if mid % 2 == 1 {
            if nums[mid] == nums[mid - 1] {
                lo = mid + 1;
            } else {
                hi = mid;
            }
        } else {
            if mid + 1 < nums.len() && nums[mid] == nums[mid + 1] {
                lo = mid + 2;
            } else {
                hi = mid;
            }
        }
    }
    nums[lo]
}

fn main() {
    println!("{}", single_non_duplicate(vec![1, 1, 2, 3, 3, 4, 4, 8, 8]));
}

#[cfg(test)]
mod tests {
    use super::single_non_duplicate;

    #[test]
    fn example_one() {
        assert_eq!(single_non_duplicate(vec![1, 1, 2, 3, 3, 4, 4, 8, 8]), 2);
    }
}
