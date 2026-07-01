/// LeetCode #2185 - Counting Words With a Given Prefix
fn prefix_count(words: Vec<String>, pref: String) -> i32 {
    words.iter().filter(|w| w.starts_with(&pref)).count() as i32
}

fn main() {
    println!(
        "{}",
        prefix_count(
            vec![
                "pay".into(),
                "attention".into(),
                "practice".into(),
                "attend".into()
            ],
            "at".into(),
        )
    );
}

#[cfg(test)]
mod tests {
    use super::prefix_count;

    #[test]
    fn example_one() {
        assert_eq!(
            prefix_count(
                vec![
                    "pay".into(),
                    "attention".into(),
                    "practice".into(),
                    "attend".into()
                ],
                "at".into(),
            ),
            2
        );
    }

    #[test]
    fn example_two() {
        assert_eq!(
            prefix_count(
                vec!["leetcode".into(), "win".into(), "loops".into()],
                "code".into()
            ),
            0
        );
    }
}
