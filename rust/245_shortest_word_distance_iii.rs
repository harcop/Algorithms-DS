/// LeetCode #245 - Shortest Word Distance III
fn shortest_word_distance(words_dict: Vec<String>, word1: String, word2: String) -> i32 {
    let mut d = words_dict.len();
    if word1 == word2 {
        let mut prev = None;
        for (i, w) in words_dict.iter().enumerate() {
            if w == &word1 {
                if let Some(j) = prev {
                    d = d.min(i - j);
                }
                prev = Some(i);
            }
        }
    } else {
        let mut p1 = None;
        let mut p2 = None;
        for (i, w) in words_dict.iter().enumerate() {
            if w == &word1 {
                p1 = Some(i);
            }
            if w == &word2 {
                p2 = Some(i);
            }
            if let (Some(a), Some(b)) = (p1, p2) {
                d = d.min(a.abs_diff(b));
            }
        }
    }
    d as i32
}

fn main() {
    println!(
        "{}",
        shortest_word_distance(
            vec![
                "practice".into(),
                "makes".into(),
                "perfect".into(),
                "coding".into(),
                "makes".into(),
            ],
            "makes".into(),
            "makes".into(),
        )
    );
}

#[cfg(test)]
mod tests {
    use super::shortest_word_distance;

    #[test]
    fn example_one() {
        assert_eq!(
            shortest_word_distance(
                vec![
                    "practice".into(),
                    "makes".into(),
                    "perfect".into(),
                    "coding".into(),
                    "makes".into(),
                ],
                "makes".into(),
                "coding".into(),
            ),
            1
        );
    }

    #[test]
    fn example_two() {
        assert_eq!(
            shortest_word_distance(
                vec![
                    "practice".into(),
                    "makes".into(),
                    "perfect".into(),
                    "coding".into(),
                    "makes".into(),
                ],
                "makes".into(),
                "makes".into(),
            ),
            3
        );
    }
}
