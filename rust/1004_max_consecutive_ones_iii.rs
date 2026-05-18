/// LeetCode #1004 - Max Consecutive Ones III
fn longest_ones(nums: Vec<i32>, k: i32) -> i32 {
    let mut left = 0usize;
    let mut zeros = 0i32;
    let mut best = 0i32;
    for right in 0..nums.len() {
        if nums[right] == 0 {
            zeros += 1;
        }
        while zeros > k {
            if nums[left] == 0 {
                zeros -= 1;
            }
            left += 1;
        }
        best = best.max((right - left + 1) as i32);
    }
    best
}

fn main() {
    println!("{}", longest_ones(vec![1, 1, 1, 0, 0, 0, 1, 1, 1, 1, 0], 2));
}

#[cfg(test)]
mod tests {
    use super::longest_ones;

    #[test]
    fn example_one() {
        assert_eq!(longest_ones(vec![1, 1, 1, 0, 0, 0, 1, 1, 1, 1, 0], 2), 6);
    }

    #[test]
    fn example_two() {
        assert_eq!(longest_ones(vec![0, 0, 1, 1, 0, 0, 1, 1, 1, 0, 1, 1, 0, 0, 0, 1, 1, 1, 1], 3), 10);
    }
}
