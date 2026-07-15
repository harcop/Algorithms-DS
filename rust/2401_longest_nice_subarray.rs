/// LeetCode #2401 - Longest Nice Subarray
fn longest_nice_subarray(nums: Vec<i32>) -> i32 {
    let mut ans = 0;
    let mut mask = 0;
    let mut l = 0;
    for (r, &x) in nums.iter().enumerate() {
        while mask & x != 0 {
            mask ^= nums[l];
            l += 1;
        }
        mask |= x;
        ans = ans.max((r - l + 1) as i32);
    }
    ans
}

fn main() {
    println!("{}", longest_nice_subarray(vec![1, 3, 8, 48, 10]));
}

#[cfg(test)]
mod tests {
    use super::longest_nice_subarray;

    #[test]
    fn example_one() {
        assert_eq!(longest_nice_subarray(vec![1, 3, 8, 48, 10]), 3);
    }

    #[test]
    fn example_two() {
        assert_eq!(longest_nice_subarray(vec![3, 1, 5, 11, 13]), 1);
    }
}
