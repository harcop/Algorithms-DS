/// LeetCode #3584 - Maximum Product of First and Last Elements of a Subsequence
fn maximum_product(nums: Vec<i32>, m: i32) -> i64 {
    let m = m as usize;
    let mut ans = i64::MIN;
    let mut mx = i32::MIN;
    let mut mi = i32::MAX;
    for i in m - 1..nums.len() {
        let x = nums[i] as i64;
        let y = nums[i + 1 - m];
        mi = mi.min(y);
        mx = mx.max(y);
        ans = ans.max(x * mi as i64).max(x * mx as i64);
    }
    ans
}

fn main() {
    println!("{}", maximum_product(vec![-1, -9, 2, 3, -2, -3, 1], 1));
}

#[cfg(test)]
mod tests {
    use super::maximum_product;

    #[test]
    fn example1() {
        assert_eq!(maximum_product(vec![-1, -9, 2, 3, -2, -3, 1], 1), 81);
    }

    #[test]
    fn example2() {
        assert_eq!(maximum_product(vec![1, 3, -5, 5, 6, -4], 3), 20);
    }

    #[test]
    fn example3() {
        assert_eq!(maximum_product(vec![2, -1, 2, -6, 5, 2, -5, 7], 2), 35);
    }
}
