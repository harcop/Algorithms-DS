/// LeetCode #1451 - Rearrange Words In A Sentence
fn arrange_words(text: String) -> String {
    let mut words: Vec<&str> = text.trim_end_matches('.').split_whitespace().collect();
    words.sort_by_key(|w| w.len());
    let mut s = words.join(" ").to_lowercase();
    if let Some(c) = s.get_mut(0..1) { c.make_ascii_uppercase(); }
    s.push('.');
    s
}
fn main() { println!("{}", arrange_words("Leetcode comes with Leetbench".into())); }
#[cfg(test)]
mod tests {
    use super::arrange_words;
    #[test]
    fn example_one() { assert_eq!(arrange_words("Leetcode comes with Leetbench".into()), "With comes leetcode leetbench.".to_string()); }
    #[test]
    fn example_two() { assert_eq!(arrange_words("To be or not to be".into()), "To be or to be not.".to_string()); }
}