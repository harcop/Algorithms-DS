/// LeetCode #2047 - Number of Valid Words in a Sentence
fn count_valid_words(sentence: String) -> i32 {
    fn check(s: &str) -> bool {
        let bytes = s.as_bytes();
        let mut hyphen = false;
        for (i, &c) in bytes.iter().enumerate() {
            if c.is_ascii_digit() || (matches!(c, b'!' | b',' | b'.') && i < bytes.len() - 1) {
                return false;
            }
            if c == b'-' {
                if hyphen
                    || i == 0
                    || i == bytes.len() - 1
                    || !bytes[i - 1].is_ascii_alphabetic()
                    || !bytes[i + 1].is_ascii_alphabetic()
                {
                    return false;
                }
                hyphen = true;
            }
        }
        true
    }

    sentence
        .split_whitespace()
        .filter(|s| check(s))
        .count() as i32
}

fn main() {
    println!("{}", count_valid_words("cat and  dog".into()));
}

#[cfg(test)]
mod tests {
    use super::count_valid_words;

    #[test]
    fn example_one() {
        assert_eq!(count_valid_words("cat and  dog".into()), 3);
    }

    #[test]
    fn example_two() {
        assert_eq!(count_valid_words("!this  1-s b8d!".into()), 0);
    }

    #[test]
    fn example_three() {
        assert_eq!(count_valid_words("alice and  bob are playing stone-game10".into()), 5);
    }
}
