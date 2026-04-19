/// LeetCode #7 - Reverse Integer
///
/// Given a signed 32-bit integer x, return x with its digits reversed.
/// If reversing causes overflow, return 0.

fn reverse(x: i32) -> i32 {
    let mut n = x as i64;
    let mut rev: i64 = 0;

    while n != 0 {
        rev = rev * 10 + n % 10;
        n /= 10;
    }

    if rev < i32::MIN as i64 || rev > i32::MAX as i64 {
        0
    } else {
        rev as i32
    }
}

fn main() {
    println!("{}", reverse(123));
}

#[cfg(test)]
mod tests {
    use super::reverse;

    #[test]
    fn example_one() {
        assert_eq!(reverse(123), 321);
    }

    #[test]
    fn example_two() {
        assert_eq!(reverse(-123), -321);
    }

    #[test]
    fn example_three() {
        assert_eq!(reverse(120), 21);
    }

    #[test]
    fn overflow_returns_zero() {
        assert_eq!(reverse(1534236469), 0);
    }
}
