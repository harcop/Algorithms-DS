/// LeetCode #1909 - Remove One Element to Make the Array Strictly Increasing
fn can_be_increasing(nums: Vec<i32>) -> bool {
    fn check(nums: &[i32], k: usize) -> bool {
        let mut pre = i32::MIN;
        for (i, &x) in nums.iter().enumerate() {
            if i == k {
                continue;
            }
            if pre >= x {
                return false;
            }
            pre = x;
        }
        true
    }

    let mut i = 0usize;
    while i + 1 < nums.len() && nums[i] < nums[i + 1] {
        i += 1;
    }
    check(&nums, i) || check(&nums, i + 1)
}

fn main() {
    println!("{}", can_be_increasing(vec![1, 2, 10, 5, 7]));
}

#[cfg(test)]
mod tests {
    use super::can_be_increasing;

    #[test]
    fn example_one() {
        assert!(can_be_increasing(vec![1, 2, 10, 5, 7]));
    }

    #[test]
    fn example_two() {
        assert!(!can_be_increasing(vec![2, 3, 1, 2]));
    }

    #[test]
    fn example_three() {
        assert!(!can_be_increasing(vec![1, 1, 1]));
    }
}
