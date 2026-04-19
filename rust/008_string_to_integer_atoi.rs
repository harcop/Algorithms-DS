/// LeetCode #8 - String to Integer (atoi)
///
/// Implement the myAtoi function: whitespace, optional sign, digits, clamp to 32-bit range.

fn my_atoi(s: String) -> i32 {
    let s = s.trim_start();
    if s.is_empty() {
        return 0;
    }

    let bytes = s.as_bytes();
    let mut i = 0usize;
    let mut sign: i64 = 1;

    match bytes[0] {
        b'+' => i = 1,
        b'-' => {
            sign = -1;
            i = 1;
        }
        _ => {}
    }

    let mut acc: i64 = 0;
    while i < bytes.len() && bytes[i].is_ascii_digit() {
        acc = acc * 10 + (bytes[i] - b'0') as i64;
        if sign * acc > i32::MAX as i64 {
            return i32::MAX;
        }
        if sign * acc < i32::MIN as i64 {
            return i32::MIN;
        }
        i += 1;
    }

    (sign * acc) as i32
}

fn main() {
    println!("{}", my_atoi("42".to_string()));
}

#[cfg(test)]
mod tests {
    use super::my_atoi;

    #[test]
    fn example_one() {
        assert_eq!(my_atoi("42".to_string()), 42);
    }

    #[test]
    fn example_two() {
        assert_eq!(my_atoi("   -42".to_string()), -42);
    }

    #[test]
    fn example_three() {
        assert_eq!(my_atoi("4193 with words".to_string()), 4193);
    }

    #[test]
    fn clamps_positive() {
        assert_eq!(
            my_atoi("91283472332".to_string()),
            2147483647
        );
    }

    #[test]
    fn clamps_negative() {
        assert_eq!(
            my_atoi("-91283472332".to_string()),
            -2147483648
        );
    }
}
