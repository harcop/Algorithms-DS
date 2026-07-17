/// LeetCode #2443 - Sum of Number and Its Reverse
fn sum_of_number_and_reverse(num: i32) -> bool {
    fn reverse(mut value: i32) -> i32 {
        let mut reversed = 0;
        while value > 0 {
            reversed = reversed * 10 + value % 10;
            value /= 10;
        }
        reversed
    }

    (0..=num).any(|value| value + reverse(value) == num)
}

fn main() {
    println!("{}", sum_of_number_and_reverse(443));
}

#[cfg(test)]
mod tests {
    use super::sum_of_number_and_reverse;

    #[test]
    fn example_one() {
        assert!(sum_of_number_and_reverse(443));
    }

    #[test]
    fn example_two() {
        assert!(!sum_of_number_and_reverse(63));
    }

    #[test]
    fn zero() {
        assert!(sum_of_number_and_reverse(0));
    }
}
