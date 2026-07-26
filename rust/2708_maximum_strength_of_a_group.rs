/// LeetCode #2708 - Maximum Strength of a Group
fn max_strength(nums: Vec<i32>) -> i64 {
    let n = nums.len();
    let mut ans = i64::MIN;
    for i in 1..(1 << n) {
        let mut t = 1i64;
        for j in 0..n {
            if (i >> j) & 1 == 1 {
                t *= nums[j] as i64;
            }
        }
        ans = ans.max(t);
    }
    ans
}

fn main() {
    println!("{}", max_strength(vec![3, -1, -5, 2, 5, -9]));
}

#[cfg(test)]
mod tests {
    use super::max_strength;

    #[test]
    fn example_one() {
        assert_eq!(max_strength(vec![3, -1, -5, 2, 5, -9]), 1350);
    }

    #[test]
    fn example_two() {
        assert_eq!(max_strength(vec![-4, -5, -4]), 20);
    }
}
