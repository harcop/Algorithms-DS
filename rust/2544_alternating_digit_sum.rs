/// LeetCode #2544 - Alternating Digit Sum
fn alternate_digit_sum(mut n: i32) -> i32 {
    let mut ans = 0;
    let mut sign = 1;
    while n > 0 {
        sign *= -1;
        ans += sign * (n % 10);
        n /= 10;
    }
    sign * ans
}

fn main() {
    println!("{}", alternate_digit_sum(521));
}

#[cfg(test)]
mod tests {
    use super::alternate_digit_sum;

    #[test]
    fn example_one() {
        assert_eq!(alternate_digit_sum(521), 4);
    }

    #[test]
    fn example_two() {
        assert_eq!(alternate_digit_sum(111), 1);
    }

    #[test]
    fn example_three() {
        assert_eq!(alternate_digit_sum(886996), 0);
    }
}
