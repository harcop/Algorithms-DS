/// LeetCode #3732 - Maximum Product of Three Elements After One Replacement
fn max_product(mut nums: Vec<i32>) -> i64 {
    nums.sort_unstable();
    let n = nums.len();
    let a = nums[0] as i64;
    let b = nums[1] as i64;
    let c = nums[n - 2] as i64;
    let d = nums[n - 1] as i64;
    let x = 100_000i64;
    (a * b * x).max(c * d * x).max(-a * d * x)
}

fn main() {
    println!("{}", max_product(vec![-5, 7, 0]));
}

#[cfg(test)]
mod tests {
    use super::max_product;

    #[test]
    fn example1() {
        assert_eq!(max_product(vec![-5, 7, 0]), 3_500_000);
    }

    #[test]
    fn example2() {
        assert_eq!(max_product(vec![-4, -2, -1, -3]), 1_200_000);
    }

    #[test]
    fn example3() {
        assert_eq!(max_product(vec![0, 10, 0]), 0);
    }
}
