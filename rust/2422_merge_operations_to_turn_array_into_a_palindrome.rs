/// LeetCode #2422 - Merge Operations to Turn Array Into a Palindrome
fn minimum_operations(nums: Vec<i32>) -> i32 {
    let mut nums: Vec<i64> = nums.into_iter().map(|x| x as i64).collect();
    let mut left = 0usize;
    let mut right = nums.len() - 1;
    let mut ans = 0;

    while left < right {
        if nums[left] == nums[right] {
            left += 1;
            right -= 1;
        } else if nums[left] < nums[right] {
            nums[left + 1] += nums[left];
            left += 1;
            ans += 1;
        } else {
            nums[right - 1] += nums[right];
            right -= 1;
            ans += 1;
        }
    }

    ans
}

fn main() {
    println!("{}", minimum_operations(vec![4, 3, 2, 1, 2, 3, 1]));
}

#[cfg(test)]
mod tests {
    use super::minimum_operations;

    #[test]
    fn example_one() {
        assert_eq!(minimum_operations(vec![4, 3, 2, 1, 2, 3, 1]), 2);
    }

    #[test]
    fn example_two() {
        assert_eq!(minimum_operations(vec![1, 2, 3, 4]), 3);
    }
}
