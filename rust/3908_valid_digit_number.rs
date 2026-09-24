/// LeetCode #3908 - Valid Digit Number
fn valid_digit(n: i32, x: i32) -> bool {
    let s = n.to_string();
    let digit = (b'0' + x as u8) as char;
    s.contains(digit) && !s.starts_with(digit)
}

fn main() {
    println!("{}", valid_digit(101, 0));
}

#[cfg(test)]
mod tests {
    use super::valid_digit;

    #[test]
    fn example1() {
        assert!(valid_digit(101, 0));
    }

    #[test]
    fn example2() {
        assert!(!valid_digit(232, 2));
    }

    #[test]
    fn example3() {
        assert!(!valid_digit(5, 1));
    }
}
