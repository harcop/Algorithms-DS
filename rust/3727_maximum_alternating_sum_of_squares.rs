/// LeetCode #3727 - Maximum Alternating Sum of Squares
fn max_alternating_sum(mut nums: Vec<i32>) -> i64 {
    for x in &mut nums {
        *x *= *x;
    }
    nums.sort_unstable();
    let m = nums.len() / 2;
    let mut ans = 0i64;
    for &x in &nums[..m] {
        ans -= x as i64;
    }
    for &x in &nums[m..] {
        ans += x as i64;
    }
    ans
}

fn main() {
    println!("{}", max_alternating_sum(vec![1, 2, 3]));
}

#[cfg(test)]
mod tests {
    use super::max_alternating_sum;

    #[test]
    fn example1() {
        assert_eq!(max_alternating_sum(vec![1, 2, 3]), 12);
    }

    #[test]
    fn example2() {
        assert_eq!(max_alternating_sum(vec![1, -1, 2, -2, 3, -3]), 16);
    }
}
