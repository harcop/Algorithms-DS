/// LeetCode #3818 - Minimum Prefix Removal to Make Array Strictly Increasing
fn minimum_prefix_length(nums: Vec<i32>) -> i32 {
    for i in (1..nums.len()).rev() {
        if nums[i - 1] >= nums[i] {
            return i as i32;
        }
    }
    0
}

fn main() {
    println!("{}", minimum_prefix_length(vec![1, -1, 2, 3, 3, 4, 5]));
}

#[cfg(test)]
mod tests {
    use super::minimum_prefix_length;

    #[test]
    fn example1() {
        assert_eq!(minimum_prefix_length(vec![1, -1, 2, 3, 3, 4, 5]), 4);
    }

    #[test]
    fn example2() {
        assert_eq!(minimum_prefix_length(vec![4, 3, -2, -5]), 3);
    }

    #[test]
    fn example3() {
        assert_eq!(minimum_prefix_length(vec![1, 2, 3, 4]), 0);
    }
}
