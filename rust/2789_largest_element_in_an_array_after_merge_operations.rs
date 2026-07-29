/// LeetCode #2789 - Largest Element in an Array after Merge Operations
fn max_array_value(nums: Vec<i32>) -> i64 {
    let n = nums.len();
    let mut ans = nums[n - 1] as i64;
    let mut t = ans;
    for i in (0..n - 1).rev() {
        if nums[i] as i64 <= t {
            t += nums[i] as i64;
        } else {
            t = nums[i] as i64;
        }
        ans = ans.max(t);
    }
    ans
}

fn main() {
    println!("{}", max_array_value(vec![2, 3, 7, 9, 3]));
}

#[cfg(test)]
mod tests {
    use super::max_array_value;

    #[test]
    fn example_one() {
        assert_eq!(max_array_value(vec![2, 3, 7, 9, 3]), 21);
    }

    #[test]
    fn example_two() {
        assert_eq!(max_array_value(vec![5, 3, 3]), 11);
    }
}
