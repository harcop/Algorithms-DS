/// LeetCode #962 - Maximum Width Ramp

fn max_width_ramp(nums: Vec<i32>) -> i32 {
    let n = nums.len();
    let mut stack = Vec::new();
    for i in 0..n {
        if stack.is_empty() || nums[*stack.last().unwrap()] > nums[i] {
            stack.push(i);
        }
    }
    let mut ans = 0i32;
    for j in (0..n).rev() {
        while let Some(&i) = stack.last() {
            if nums[i] <= nums[j] {
                ans = ans.max((j - i) as i32);
                stack.pop();
            } else {
                break;
            }
        }
    }
    ans
}

fn main() {
    println!("{}", max_width_ramp(vec![6, 0, 8, 2, 1, 5]));
}

#[cfg(test)]
mod tests {
    use super::max_width_ramp;

    #[test]
    fn example_one() {
        assert_eq!(max_width_ramp(vec![6, 0, 8, 2, 1, 5]), 4);
    }

    #[test]
    fn example_two() {
        assert_eq!(max_width_ramp(vec![9, 8, 1, 0, 1, 9, 4, 0, 4, 1]), 7);
    }
}
