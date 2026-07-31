/// LeetCode #2828 - Check if a String Is an Acronym of Words
fn is_acronym(words: Vec<String>, s: String) -> bool {
    let acronym: String = words.iter().filter_map(|w| w.chars().next()).collect();
    acronym == s
}

fn main() {
    println!(
        "{}",
        is_acronym(
            vec!["alice".into(), "bob".into(), "charlie".into()],
            "abc".into()
        )
    );
}

#[cfg(test)]
mod tests {
    use super::is_acronym;

    #[test]
    fn example_one() {
        assert!(is_acronym(
            vec!["alice".into(), "bob".into(), "charlie".into()],
            "abc".into()
        ));
    }

    #[test]
    fn example_two() {
        assert!(!is_acronym(vec!["an".into(), "apple".into()], "a".into()));
    }

    #[test]
    fn example_three() {
        assert!(is_acronym(
            vec![
                "never".into(),
                "gonna".into(),
                "give".into(),
                "up".into(),
                "on".into(),
                "you".into()
            ],
            "ngguoy".into()
        ));
    }
}
