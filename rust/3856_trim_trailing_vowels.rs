/// LeetCode #3856 - Trim Trailing Vowels
fn trim_trailing_vowels(s: String) -> String {
    let bytes = s.as_bytes();
    let mut i = bytes.len() as i32 - 1;
    while i >= 0 && matches!(bytes[i as usize], b'a' | b'e' | b'i' | b'o' | b'u') {
        i -= 1;
    }
    s[..(i + 1) as usize].to_string()
}

fn main() {
    println!("{}", trim_trailing_vowels("idea".into()));
}

#[cfg(test)]
mod tests {
    use super::trim_trailing_vowels;

    #[test]
    fn example1() {
        assert_eq!(trim_trailing_vowels("idea".into()), "id");
    }

    #[test]
    fn example2() {
        assert_eq!(trim_trailing_vowels("day".into()), "day");
    }

    #[test]
    fn example3() {
        assert_eq!(trim_trailing_vowels("aeiou".into()), "");
    }
}
