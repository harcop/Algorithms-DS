/// LeetCode #3974 - Maximum Total Sum of K Selected Elements
fn max_sum(mut nums: Vec<i32>, k: i32, mut mul: i32) -> i64 {
    nums.sort_unstable();
    let n = nums.len();
    let k = k as usize;
    let mut ans = 0i64;
    for i in (n - k..n).rev() {
        ans += nums[i] as i64 * mul.max(1) as i64;
        mul -= 1;
    }
    ans
}

fn main() {
    println!("{}", max_sum(vec![6, 1, 2, 9], 3, 2));
}

#[cfg(test)]
mod tests {
    use super::max_sum;

    #[test]
    fn example1() {
        assert_eq!(max_sum(vec![6, 1, 2, 9], 3, 2), 26);
    }

    #[test]
    fn example2() {
        assert_eq!(max_sum(vec![3, 7, 5, 2], 2, 4), 43);
    }

    #[test]
    fn example3() {
        assert_eq!(max_sum(vec![4, 4], 1, 1), 4);
    }
}
