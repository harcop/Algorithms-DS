/// LeetCode #1455 - Check If A Word Occurs As A Prefix Of Any Word In A Sentence
fn is_prefix_of_word(sentence: String, search_word: String) -> i32 {
    for (i, w) in sentence.split_whitespace().enumerate() {
        if w.starts_with(&search_word) { return (i + 1) as i32; }
    }
    -1
}
fn main() { println!("{}", is_prefix_of_word("i love eating burger".into(), "burg".into())); }
#[cfg(test)]
mod tests {
    use super::is_prefix_of_word;
    #[test]
    fn example_one() { assert_eq!(is_prefix_of_word("i love eating burger".into(), "burg".into()), 4); }
    #[test]
    fn example_two() { assert_eq!(is_prefix_of_word("this problem is an easy problem".into(), "pro".into()), 2); }
}