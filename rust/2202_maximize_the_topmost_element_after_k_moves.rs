/// LeetCode #2202 - Maximize the Topmost Element After K Moves
fn maximum_top(nums: Vec<i32>, k: i32) -> i32 {
    if k == 0 {
        return nums[0];
    }
    let n = nums.len();
    if n == 1 {
        if k % 2 == 1 {
            return -1;
        }
        return nums[0];
    }

    let mut ans = -1;
    for &x in nums.iter().take((k - 1) as usize) {
        ans = ans.max(x);
    }
    if (k as usize) < n {
        ans = ans.max(nums[k as usize]);
    }
    ans
}

fn main() {
    println!("{}", maximum_top(vec![5, 2, 2, 4, 0, 6], 4));
}

#[cfg(test)]
mod tests {
    use super::maximum_top;

    #[test]
    fn example_one() {
        assert_eq!(maximum_top(vec![5, 2, 2, 4, 0, 6], 4), 5);
    }

    #[test]
    fn example_two() {
        assert_eq!(maximum_top(vec![2], 1), -1);
    }
}
