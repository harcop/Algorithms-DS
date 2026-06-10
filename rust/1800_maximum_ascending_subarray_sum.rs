/// LeetCode #1800 - Maximum Ascending Subarray Sum
fn max_ascending_sum(nums: Vec<i32>) -> i32 {
    let mut ans = 0i32;
    let mut t = 0i32;
    for (i, &v) in nums.iter().enumerate() {
        if i == 0 || v > nums[i - 1] {
            t += v;
            ans = ans.max(t);
        } else {
            t = v;
        }
    }
    ans
}

fn main() {
    println!("{}", max_ascending_sum(vec![10, 20, 30, 5, 10, 50]));
}

#[cfg(test)]
mod tests {
    use super::max_ascending_sum;

    #[test]
    fn example_one() {
        assert_eq!(max_ascending_sum(vec![10, 20, 30, 5, 10, 50]), 65);
    }

    #[test]
    fn example_two() {
        assert_eq!(max_ascending_sum(vec![10, 20, 30, 40, 50]), 150);
    }
}
