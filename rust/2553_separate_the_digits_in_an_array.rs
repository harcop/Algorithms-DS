/// LeetCode #2553 - Separate the Digits in an Array
fn separate_digits(nums: Vec<i32>) -> Vec<i32> {
    let mut ans = Vec::new();
    for mut num in nums {
        let mut digits = Vec::new();
        while num > 0 {
            digits.push(num % 10);
            num /= 10;
        }
        digits.reverse();
        ans.extend(digits);
    }
    ans
}

fn main() {
    println!("{:?}", separate_digits(vec![13, 25, 83, 77]));
}

#[cfg(test)]
mod tests {
    use super::separate_digits;

    #[test]
    fn example_one() {
        assert_eq!(
            separate_digits(vec![13, 25, 83, 77]),
            vec![1, 3, 2, 5, 8, 3, 7, 7]
        );
    }

    #[test]
    fn example_two() {
        assert_eq!(separate_digits(vec![7, 1, 3, 9]), vec![7, 1, 3, 9]);
    }
}
