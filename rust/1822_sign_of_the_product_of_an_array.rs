/// LeetCode #1822 - Sign of the Product of an Array
fn array_sign(nums: Vec<i32>) -> i32 {
    let mut ans = 1i32;
    for v in nums {
        if v == 0 {
            return 0;
        }
        if v < 0 {
            ans = -ans;
        }
    }
    ans
}

fn main() {
    println!("{}", array_sign(vec![-1, -2, -3, -4, 3, 2, 1]));
}

#[cfg(test)]
mod tests {
    use super::array_sign;

    #[test]
    fn example_one() {
        assert_eq!(array_sign(vec![-1, -2, -3, -4, 3, 2, 1]), 1);
    }

    #[test]
    fn example_two() {
        assert_eq!(array_sign(vec![1, 5, 0, 2, -3]), 0);
    }
}
