/// LeetCode #665 - Non-decreasing Array
fn check_possibility(nums: Vec<i32>) -> bool {
    let mut nums = nums;
    let mut modified = 0;
    for i in 1..nums.len() {
        if nums[i] < nums[i - 1] {
            modified += 1;
            if modified > 1 { return false; }
            if i < 2 || nums[i - 2] <= nums[i] {
                nums[i - 1] = nums[i];
            } else {
                nums[i] = nums[i - 1];
            }
        }
    }
    true
}

fn main() {
    println!("{}", check_possibility(vec![4, 2, 3]));
}

#[cfg(test)]
mod tests {
    use super::check_possibility;

    #[test]
    fn example_one() {
        assert!(check_possibility(vec![4, 2, 3]));
    }

    #[test]
    fn example_two() {
        assert!(!check_possibility(vec![4, 2, 1]));
    }
}
