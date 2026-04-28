/// LeetCode #53 - Maximum Subarray
fn max_sub_array(nums: Vec<i32>) -> i32 {
    let mut best = nums[0];
    let mut current = nums[0];

    for &num in nums.iter().skip(1) {
        current = num.max(current + num);
        best = best.max(current);
    }

    best
}

fn main() {
    println!("{}", max_sub_array(vec![-2, 1, -3, 4, -1, 2, 1, -5, 4]));
}

#[cfg(test)]
mod tests {
    use super::max_sub_array;

    #[test]
    fn example_one() {
        assert_eq!(max_sub_array(vec![-2, 1, -3, 4, -1, 2, 1, -5, 4]), 6);
    }

    #[test]
    fn example_two() {
        assert_eq!(max_sub_array(vec![1]), 1);
    }

    #[test]
    fn example_three() {
        assert_eq!(max_sub_array(vec![5, 4, -1, 7, 8]), 23);
    }
}
