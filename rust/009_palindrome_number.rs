/// LeetCode #9 - Palindrome Number
///
/// Return true if x reads the same forward and backward. Negative numbers are not palindromes.

fn is_palindrome(x: i32) -> bool {
    if x < 0 {
        return false;
    }
    if x != 0 && x % 10 == 0 {
        return false;
    }

    let mut n = x;
    let mut rev_half = 0;

    while n > rev_half {
        rev_half = rev_half * 10 + n % 10;
        n /= 10;
    }

    n == rev_half || n == rev_half / 10
}

fn main() {
    println!("{}", is_palindrome(121));
}

#[cfg(test)]
mod tests {
    use super::is_palindrome;

    #[test]
    fn example_one() {
        assert!(is_palindrome(121));
    }

    #[test]
    fn example_two() {
        assert!(!is_palindrome(-121));
    }

    #[test]
    fn example_three() {
        assert!(!is_palindrome(10));
    }

    #[test]
    fn zero() {
        assert!(is_palindrome(0));
    }
}
