/// LeetCode #2847 - Smallest Number With Given Digit Product
fn smallest_number(mut n: i64) -> String {
    if n < 10 {
        return n.to_string();
    }

    let mut digits = Vec::new();
    for digit in (2..=9).rev() {
        while n % digit == 0 {
            digits.push(digit);
            n /= digit;
        }
    }
    if n != 1 {
        return "-1".into();
    }
    digits.reverse();
    digits
        .into_iter()
        .map(|digit| char::from(b'0' + digit as u8))
        .collect()
}

fn main() {
    println!("{}", smallest_number(105));
}

#[cfg(test)]
mod tests {
    use super::smallest_number;

    #[test]
    fn example_one() {
        assert_eq!(smallest_number(105), "357");
    }

    #[test]
    fn example_two() {
        assert_eq!(smallest_number(7), "7");
    }

    #[test]
    fn example_three() {
        assert_eq!(smallest_number(44), "-1");
    }
}
