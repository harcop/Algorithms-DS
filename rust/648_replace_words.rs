/// LeetCode #648 - Replace Words
use std::collections::HashSet;

fn replace_words(dictionary: Vec<String>, sentence: String) -> String {
    let set: HashSet<&str> = dictionary.iter().map(|s| s.as_str()).collect();
    let words: Vec<String> = sentence.split_whitespace().map(|w| {
        let mut best: Option<&str> = None;
        for end in 1..=w.len() {
            let prefix = &w[..end];
            if set.contains(prefix) {
                best = Some(prefix);
                break;
            }
        }
        best.map(|s| s.to_string()).unwrap_or_else(|| w.to_string())
    }).collect();
    words.join(" ")
}

fn main() {
    println!("{}", replace_words(vec!["cat".into(), "bat".into(), "rat".into()], "the cattle was rattled by the battery".into()));
}

#[cfg(test)]
mod tests {
    use super::replace_words;

    #[test]
    fn example_one() {
        assert_eq!(
            replace_words(vec!["cat".into(), "bat".into(), "rat".into()], "the cattle was rattled by the battery".into()),
            "the cat was rat by the bat"
        );
    }
}
