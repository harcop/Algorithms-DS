/// LeetCode #1935 - Maximum Number of Words You Can Type
fn can_be_typed_words(text: String, broken_letters: String) -> i32 {
    let mut bad = [false; 26];
    for b in broken_letters.bytes() {
        bad[(b - b'a') as usize] = true;
    }
    text.split_whitespace()
        .filter(|word| !word.bytes().any(|b| bad[(b - b'a') as usize]))
        .count() as i32
}

fn main() {
    println!("{}", can_be_typed_words("hello world".into(), "ld".into()));
}

#[cfg(test)]
mod tests {
    use super::can_be_typed_words;

    #[test]
    fn example_one() {
        assert_eq!(can_be_typed_words("hello world".into(), "ad".into()), 1);
    }

    #[test]
    fn example_two() {
        assert_eq!(can_be_typed_words("leet code".into(), "lt".into()), 1);
    }

    #[test]
    fn example_three() {
        assert_eq!(can_be_typed_words("leet code".into(), "e".into()), 0);
    }
}
