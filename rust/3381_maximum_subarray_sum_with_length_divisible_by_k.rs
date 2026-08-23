/// LeetCode #3381 - Maximum Subarray Sum With Length Divisible by K
fn max_subarray_sum(nums: Vec<i32>, k: i32) -> i64 {
    let k = k as usize;
    let inf = i64::MAX / 4;
    let mut f = vec![inf; k];
    let mut ans = -inf;
    let mut s = 0i64;
    f[k - 1] = 0;
    for (i, &x) in nums.iter().enumerate() {
        s += x as i64;
        ans = ans.max(s - f[i % k]);
        f[i % k] = f[i % k].min(s);
    }
    ans
}

fn main() {
    println!("{}", max_subarray_sum(vec![1, 2], 1));
}

#[cfg(test)]
mod tests {
    use super::max_subarray_sum;

    #[test]
    fn example1() {
        assert_eq!(max_subarray_sum(vec![1, 2], 1), 3);
    }

    #[test]
    fn example2() {
        assert_eq!(max_subarray_sum(vec![-1, -2, -3, -4, -5], 4), -10);
    }

    #[test]
    fn example3() {
        assert_eq!(max_subarray_sum(vec![-5, 1, 2, -3, 4], 2), 4);
    }
}
