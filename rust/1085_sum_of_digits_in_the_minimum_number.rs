/// LeetCode #1085 - Sum of Digits in the Minimum Number
fn sum_of_digits(nums: Vec<i32>) -> i32 {
    let min = *nums.iter().min().unwrap();
    let sum: i32 = min
        .to_string()
        .bytes()
        .map(|c| (c - b'0') as i32)
        .sum();
    if sum % 2 == 1 {
        1
    } else {
        0
    }
}

fn main() {
    println!("{}", sum_of_digits(vec![34, 23, 1, 24, 75, 33, 54, 8]));
}

#[cfg(test)]
mod tests {
    use super::sum_of_digits;

    #[test]
    fn example_one() {
        assert_eq!(sum_of_digits(vec![34, 23, 1, 24, 75, 33, 54, 8]), 1);
    }

    #[test]
    fn example_two() {
        assert_eq!(sum_of_digits(vec![99, 77, 33, 66, 55]), 0);
    }
}
