/// LeetCode #3136 - Valid Word
fn is_valid(word: String) -> bool {
    if word.len() < 3 {
        return false;
    }
    let vowels = b"aeiouAEIOU";
    let mut has_vowel = false;
    let mut has_consonant = false;
    for c in word.bytes() {
        if !(c.is_ascii_alphanumeric()) {
            return false;
        }
        if c.is_ascii_alphabetic() {
            if vowels.contains(&c) {
                has_vowel = true;
            } else {
                has_consonant = true;
            }
        }
    }
    has_vowel && has_consonant
}

fn main() {
    println!("{}", is_valid("234Adas".into()));
}

#[cfg(test)]
mod tests {
    use super::is_valid;

    #[test]
    fn example1() {
        assert!(is_valid("234Adas".into()));
    }

    #[test]
    fn example2() {
        assert!(!is_valid("b3".into()));
    }

    #[test]
    fn example3() {
        assert!(!is_valid("a3$e".into()));
    }
}
