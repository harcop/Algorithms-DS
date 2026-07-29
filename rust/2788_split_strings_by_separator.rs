/// LeetCode #2788 - Split Strings by Separator
fn split_words_by_separator(words: Vec<String>, separator: char) -> Vec<String> {
    words
        .iter()
        .flat_map(|w| w.split(separator).filter(|s| !s.is_empty()))
        .map(|s| s.to_string())
        .collect()
}

fn main() {
    println!(
        "{:?}",
        split_words_by_separator(
            vec![
                "one.two.three".into(),
                "four.five".into(),
                "six".into(),
            ],
            '.'
        )
    );
}

#[cfg(test)]
mod tests {
    use super::split_words_by_separator;

    #[test]
    fn example_one() {
        assert_eq!(
            split_words_by_separator(
                vec![
                    "one.two.three".into(),
                    "four.five".into(),
                    "six".into(),
                ],
                '.'
            ),
            vec![
                "one".to_string(),
                "two".to_string(),
                "three".to_string(),
                "four".to_string(),
                "five".to_string(),
                "six".to_string(),
            ]
        );
    }

    #[test]
    fn example_two() {
        assert_eq!(
            split_words_by_separator(vec!["$easy$".into(), "$problem$".into()], '$'),
            vec!["easy".to_string(), "problem".to_string()]
        );
    }

    #[test]
    fn example_three() {
        assert_eq!(
            split_words_by_separator(vec!["|||".into()], '|'),
            Vec::<String>::new()
        );
    }
}
