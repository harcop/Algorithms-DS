/// LeetCode #581 - Shortest Unsorted Continuous Subarray
fn find_unsorted_subarray(nums: Vec<i32>) -> i32 {
    let n = nums.len();
    if n <= 1 {
        return 0;
    }
    let mut min_v = nums[n - 1];
    let mut begin = n;
    for i in (0..n).rev() {
        if nums[i] > min_v {
            begin = i;
        } else {
            min_v = min_v.min(nums[i]);
        }
    }
    let mut max_v = nums[0];
    let mut end = 0usize;
    for i in 0..n {
        if nums[i] < max_v {
            end = i;
        } else {
            max_v = max_v.max(nums[i]);
        }
    }
    if begin >= end {
        0
    } else {
        (end - begin + 1) as i32
    }
}

fn main() {
    println!("{}", find_unsorted_subarray(vec![2, 6, 4, 8, 10, 9, 15]));
}

#[cfg(test)]
mod tests {
    use super::find_unsorted_subarray;

    #[test]
    fn example_one() {
        assert_eq!(find_unsorted_subarray(vec![2, 6, 4, 8, 10, 9, 15]), 5);
    }

    #[test]
    fn example_two() {
        assert_eq!(find_unsorted_subarray(vec![1, 2, 3, 4, 5]), 0);
    }
}
