/// LeetCode #2710 - Remove Trailing Zeros From a String
fn remove_trailing_zeros(num: String) -> String {
    let bytes = num.as_bytes();
    let mut i = bytes.len();
    while i > 0 && bytes[i - 1] == b'0' {
        i -= 1;
    }
    num[..i].to_string()
}

fn main() {
    println!("{}", remove_trailing_zeros("51230100".into()));
}

#[cfg(test)]
mod tests {
    use super::remove_trailing_zeros;

    #[test]
    fn example_one() {
        assert_eq!(remove_trailing_zeros("51230100".into()), "512301");
    }

    #[test]
    fn example_two() {
        assert_eq!(remove_trailing_zeros("123".into()), "123");
    }
}
