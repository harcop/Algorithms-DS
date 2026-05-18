/// LeetCode #910 - Smallest Range II
fn smallest_range_ii(mut nums: Vec<i32>, k: i32) -> i32 {
    nums.sort_unstable();
    let n = nums.len();
    let mut ans = nums[n - 1] - nums[0];
    for i in 0..n - 1 {
        let hi = (nums[n - 1] - k).max(nums[i] + k);
        let lo = (nums[0] + k).min(nums[i + 1] - k);
        ans = ans.min(hi - lo);
    }
    ans
}

fn main() {
    println!("{}", smallest_range_ii(vec![1], 0));
}

#[cfg(test)]
mod tests {
    use super::smallest_range_ii;

    #[test]
    fn example_one() {
        assert_eq!(smallest_range_ii(vec![1], 0), 0);
    }

    #[test]
    fn example_two() {
        assert_eq!(smallest_range_ii(vec![0, 10], 2), 6);
    }

    #[test]
    fn example_three() {
        assert_eq!(smallest_range_ii(vec![1, 3, 6], 3), 3);
    }
}
