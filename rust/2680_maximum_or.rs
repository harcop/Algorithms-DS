/// LeetCode #2680 - Maximum OR
fn maximum_or(nums: Vec<i32>, k: i32) -> i64 {
    let n = nums.len();
    let mut suf = vec![0i64; n + 1];
    for i in (0..n).rev() {
        suf[i] = suf[i + 1] | nums[i] as i64;
    }
    let mut ans = 0i64;
    let mut pre = 0i64;
    let k = k as i64;
    for i in 0..n {
        ans = ans.max(pre | ((nums[i] as i64) << k) | suf[i + 1]);
        pre |= nums[i] as i64;
    }
    ans
}

fn main() {
    println!("{}", maximum_or(vec![12, 9], 1));
}

#[cfg(test)]
mod tests {
    use super::maximum_or;

    #[test]
    fn example_one() {
        assert_eq!(maximum_or(vec![12, 9], 1), 30);
    }

    #[test]
    fn example_two() {
        assert_eq!(maximum_or(vec![8, 1, 2], 2), 35);
    }
}
