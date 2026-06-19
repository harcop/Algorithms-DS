/// LeetCode #1967 - Number of Strings That Appear as Substrings in Word
fn num_of_strings(patterns: Vec<String>, word: String) -> i32 {
    patterns
        .iter()
        .filter(|p| word.contains(p.as_str()))
        .count() as i32
}

fn main() {
    println!(
        "{}",
        num_of_strings(vec!["a".into(), "abc".into(), "bc".into()], "abc".into())
    );
}

#[cfg(test)]
mod tests {
    use super::num_of_strings;

    #[test]
    fn example_one() {
        assert_eq!(
            num_of_strings(vec!["a".into(), "abc".into(), "bc".into()], "abc".into()),
            3
        );
    }

    #[test]
    fn example_two() {
        assert_eq!(
            num_of_strings(vec!["a".into(), "b".into(), "c".into()], "aaaaabbbbb".into()),
            2
        );
    }
}
