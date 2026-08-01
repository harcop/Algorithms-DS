/// LeetCode #2855 - Minimum Right Shifts to Sort the Array
fn minimum_right_shifts(nums: Vec<i32>) -> i32 {
    let n = nums.len();
    let mut break_index = 1;
    while break_index < n && nums[break_index - 1] < nums[break_index] {
        break_index += 1;
    }
    if break_index == n {
        return 0;
    }

    for index in break_index + 1..n {
        if nums[index - 1] >= nums[index] {
            return -1;
        }
    }
    if nums[n - 1] >= nums[0] {
        return -1;
    }
    (n - break_index) as i32
}

fn main() {
    println!("{}", minimum_right_shifts(vec![3, 4, 5, 1, 2]));
}

#[cfg(test)]
mod tests {
    use super::minimum_right_shifts;

    #[test]
    fn example_one() {
        assert_eq!(minimum_right_shifts(vec![3, 4, 5, 1, 2]), 2);
    }

    #[test]
    fn example_two() {
        assert_eq!(minimum_right_shifts(vec![1, 3, 5]), 0);
    }

    #[test]
    fn example_three() {
        assert_eq!(minimum_right_shifts(vec![2, 1, 4]), -1);
    }
}
