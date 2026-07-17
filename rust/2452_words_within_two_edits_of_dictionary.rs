/// LeetCode #2452 - Words Within Two Edits of Dictionary
fn two_edit_words(queries: Vec<String>, dictionary: Vec<String>) -> Vec<String> {
    queries
        .into_iter()
        .filter(|query| {
            dictionary.iter().any(|word| {
                query
                    .bytes()
                    .zip(word.bytes())
                    .filter(|(a, b)| a != b)
                    .count()
                    <= 2
            })
        })
        .collect()
}

fn main() {
    println!(
        "{:?}",
        two_edit_words(
            vec![
                "word".to_string(),
                "note".to_string(),
                "ants".to_string(),
                "wood".to_string()
            ],
            vec!["wood".to_string(), "joke".to_string(), "moat".to_string()]
        )
    );
}

#[cfg(test)]
mod tests {
    use super::two_edit_words;

    #[test]
    fn example_one() {
        assert_eq!(
            two_edit_words(
                vec![
                    "word".to_string(),
                    "note".to_string(),
                    "ants".to_string(),
                    "wood".to_string()
                ],
                vec!["wood".to_string(), "joke".to_string(), "moat".to_string()]
            ),
            vec!["word".to_string(), "note".to_string(), "wood".to_string()]
        );
    }

    #[test]
    fn example_two() {
        assert_eq!(
            two_edit_words(vec!["yes".to_string()], vec!["not".to_string()]),
            Vec::<String>::new()
        );
    }
}
