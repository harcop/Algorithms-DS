/// LeetCode #961 - N-Repeated Element in Size 2N Array

fn repeated_n_times(nums: Vec<i32>) -> i32 {
    let n = nums.len();
    for i in 0..n - 1 {
        if nums[i] == nums[i + 1] {
            return nums[i];
        }
    }
    for i in 0..n - 2 {
        if nums[i] == nums[i + 2] {
            return nums[i];
        }
    }
    nums[0]
}

fn main() {
    println!("{}", repeated_n_times(vec![1, 2, 3, 3]));
}

#[cfg(test)]
mod tests {
    use super::repeated_n_times;

    #[test]
    fn example_one() {
        assert_eq!(repeated_n_times(vec![1, 2, 3, 3]), 3);
    }

    #[test]
    fn example_two() {
        assert_eq!(repeated_n_times(vec![2, 1, 2, 5, 3, 2]), 2);
    }

    #[test]
    fn example_three() {
        assert_eq!(repeated_n_times(vec![5, 1, 5, 2, 5, 3, 5, 4]), 5);
    }
}
