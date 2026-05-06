/// LeetCode #376 - Wiggle Subsequence
fn wiggle_max_length(nums: Vec<i32>) -> i32 {
    if nums.is_empty() {
        return 0;
    }
    let mut up = 1i32;
    let mut down = 1i32;
    for i in 1..nums.len() {
        if nums[i] > nums[i - 1] {
            up = down + 1;
        } else if nums[i] < nums[i - 1] {
            down = up + 1;
        }
    }
    up.max(down)
}

fn main() {
    println!("{}", wiggle_max_length(vec![1, 7, 4, 9, 2, 5]));
}

#[cfg(test)]
mod tests {
    use super::wiggle_max_length;

    #[test]
    fn example_one() {
        assert_eq!(wiggle_max_length(vec![1, 7, 4, 9, 2, 5]), 6);
    }
}
