/// LeetCode #3774 - Absolute Difference Between Maximum and Minimum K Elements
fn abs_difference(mut nums: Vec<i32>, k: i32) -> i32 {
    nums.sort_unstable();
    let n = nums.len();
    let k = k as usize;
    let mut ans = 0;
    for i in 0..k {
        ans += nums[n - i - 1] - nums[i];
    }
    ans
}

fn main() {
    println!("{}", abs_difference(vec![5, 2, 2, 4], 2));
}

#[cfg(test)]
mod tests {
    use super::abs_difference;

    #[test]
    fn example1() {
        assert_eq!(abs_difference(vec![5, 2, 2, 4], 2), 5);
    }

    #[test]
    fn example2() {
        assert_eq!(abs_difference(vec![100], 1), 0);
    }
}
