/// LeetCode #2104 - Sum of Subarray Ranges
fn sub_array_ranges(nums: Vec<i32>) -> i64 {
    fn contribution(nums: &[i32], is_min: bool) -> i64 {
        let n = nums.len();
        let mut stack = Vec::<usize>::new();
        let mut ans = 0i64;

        for i in 0..=n {
            while let Some(&j) = stack.last() {
                let should_pop = if i == n {
                    true
                } else if is_min {
                    nums[j] > nums[i]
                } else {
                    nums[j] < nums[i]
                };
                if !should_pop {
                    break;
                }
                stack.pop();
                let left = stack.last().map_or(j + 1, |&k| j - k);
                let right = i - j;
                ans += nums[j] as i64 * left as i64 * right as i64;
            }
            stack.push(i);
        }

        ans
    }

    contribution(&nums, false) - contribution(&nums, true)
}

fn main() {
    println!("{}", sub_array_ranges(vec![1, 2, 3]));
}

#[cfg(test)]
mod tests {
    use super::sub_array_ranges;

    #[test]
    fn example_one() {
        assert_eq!(sub_array_ranges(vec![1, 2, 3]), 4);
    }

    #[test]
    fn example_two() {
        assert_eq!(sub_array_ranges(vec![1, 3, 3]), 4);
    }

    #[test]
    fn example_three() {
        assert_eq!(sub_array_ranges(vec![4, -2, -3, 4, 1]), 59);
    }
}
