/// LeetCode #1844 - Replace All Digits with Characters
fn replace_digits(s: String) -> String {
    let mut b = s.into_bytes();
    for i in (1..b.len()).step_by(2) {
        b[i] = b[i - 1] + (b[i] - b'0');
    }
    String::from_utf8(b).unwrap()
}

fn main() {
    println!("{}", replace_digits("a1c1e1".into()));
}

#[cfg(test)]
mod tests {
    use super::replace_digits;

    #[test]
    fn example_one() {
        assert_eq!(replace_digits("a1c1e1".into()), "abcdef");
    }

    #[test]
    fn example_two() {
        assert_eq!(replace_digits("a1b2c3d4e".into()), "abbdcfdhe");
    }
}
