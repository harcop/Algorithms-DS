/// LeetCode #2114 - Maximum Number of Words Found in Sentences
fn most_words_found(sentences: Vec<String>) -> i32 {
    sentences
        .iter()
        .map(|s| s.split_whitespace().count() as i32)
        .max()
        .unwrap_or(0)
}

fn main() {
    println!(
        "{}",
        most_words_found(vec![
            "alice and bob love leetcode".into(),
            "i think so too".into(),
            "this is great thanks very much".into(),
        ])
    );
}

#[cfg(test)]
mod tests {
    use super::most_words_found;

    #[test]
    fn example_one() {
        assert_eq!(
            most_words_found(vec![
                "alice and bob love leetcode".into(),
                "i think so too".into(),
                "this is great thanks very much".into(),
            ]),
            6
        );
    }

    #[test]
    fn example_two() {
        assert_eq!(
            most_words_found(vec![
                "please wait".into(),
                "continue to fight".into(),
                "continue to win".into(),
            ]),
            3
        );
    }
}
