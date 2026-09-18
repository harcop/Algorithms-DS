/// LeetCode #3788 - Maximum Score of a Split
fn maximum_score(nums: Vec<i32>) -> i64 {
    let n = nums.len();
    let mut suf = vec![0i64; n];
    suf[n - 1] = nums[n - 1] as i64;
    for i in (0..n - 1).rev() {
        suf[i] = (nums[i] as i64).min(suf[i + 1]);
    }
    let mut ans = i64::MIN;
    let mut pre = 0i64;
    for i in 0..n - 1 {
        pre += nums[i] as i64;
        ans = ans.max(pre - suf[i + 1]);
    }
    ans
}

fn main() {
    println!("{}", maximum_score(vec![10, -1, 3, -4, -5]));
}

#[cfg(test)]
mod tests {
    use super::maximum_score;

    #[test]
    fn example1() {
        assert_eq!(maximum_score(vec![10, -1, 3, -4, -5]), 17);
    }

    #[test]
    fn example2() {
        assert_eq!(maximum_score(vec![-7, -5, 3]), -2);
    }

    #[test]
    fn example3() {
        assert_eq!(maximum_score(vec![1, 1]), 0);
    }
}
