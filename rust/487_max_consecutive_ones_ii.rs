/// LeetCode #487 - Max Consecutive Ones II
fn find_max_consecutive_ones(nums: Vec<i32>) -> i32 {
    let mut left = 0usize;
    let mut zeros = 0;
    let mut ans = 0usize;
    for right in 0..nums.len() {
        if nums[right] == 0 {
            zeros += 1;
        }
        while zeros > 1 {
            if nums[left] == 0 {
                zeros -= 1;
            }
            left += 1;
        }
        ans = ans.max(right - left + 1);
    }
    ans as i32
}

fn main() {
    println!("{}", find_max_consecutive_ones(vec![1, 0, 1, 1, 0]));
}

#[cfg(test)]
mod tests {
    use super::find_max_consecutive_ones;

    #[test]
    fn example_one() {
        assert_eq!(find_max_consecutive_ones(vec![1, 0, 1, 1, 0]), 4);
    }

    #[test]
    fn example_two() {
        assert_eq!(find_max_consecutive_ones(vec![1, 0, 1, 1, 0, 1]), 4);
    }
}
