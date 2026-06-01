/// LeetCode #1662 - Check If Two String Arrays Are Equivalent
fn array_strings_are_equal(word1: Vec<String>, word2: Vec<String>) -> bool {
    word1.concat() == word2.concat()
}
fn main() { println!("{}", array_strings_are_equal(vec!["ab".into(),"c".into()], vec!["a".into(),"bc".into()])); }
#[cfg(test)]
mod tests {
    use super::array_strings_are_equal;
    #[test]
    fn example_one() { assert!(array_strings_are_equal(vec!["ab".into(),"c".into()], vec!["a".into(),"bc".into()])); }
    #[test]
    fn example_two() { assert!(!array_strings_are_equal(vec!["a".into(),"b".into()], vec!["a".into(),"b".into(),"c".into()])); }
}