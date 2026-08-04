/// LeetCode #2972 - Count the Number of Incremovable Subarrays II
fn incremovable_subarray_count(nums: Vec<i32>) -> i64 {
    let n = nums.len();
    let mut i = 0usize;
    while i + 1 < n && nums[i] < nums[i + 1] {
        i += 1;
    }
    if i == n - 1 {
        return (n * (n + 1) / 2) as i64;
    }
    let mut ans = (i + 2) as i64;
    let mut j = n - 1;
    loop {
        while i != usize::MAX && nums[i] >= nums[j] {
            if i == 0 {
                i = usize::MAX;
                break;
            }
            i -= 1;
        }
        ans += if i == usize::MAX { 1 } else { (i + 2) as i64 };
        if j == 0 || nums[j - 1] >= nums[j] {
            break;
        }
        j -= 1;
    }
    ans
}

fn main() {
    println!("{}", incremovable_subarray_count(vec![1, 2, 3, 4]));
}

#[cfg(test)]
mod tests {
    use super::incremovable_subarray_count;

    #[test]
    fn example_one() {
        assert_eq!(incremovable_subarray_count(vec![1, 2, 3, 4]), 10);
    }

    #[test]
    fn example_two() {
        assert_eq!(incremovable_subarray_count(vec![6, 5, 7, 8]), 7);
    }

    #[test]
    fn example_three() {
        assert_eq!(incremovable_subarray_count(vec![8, 7, 6, 6]), 3);
    }
}
