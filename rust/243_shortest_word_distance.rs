/// LeetCode #243 - Shortest Word Distance
fn shortest_distance(words_dict: Vec<String>, word1: String, word2: String) -> i32 {
    let mut best = words_dict.len();
    if word1 == word2 {
        let mut prev = None;
        for (i, w) in words_dict.iter().enumerate() {
            if w == &word1 {
                if let Some(j) = prev {
                    best = best.min(i - j);
                }
                prev = Some(i);
            }
        }
    } else {
        let mut i1: Option<usize> = None;
        let mut i2: Option<usize> = None;
        for (i, w) in words_dict.iter().enumerate() {
            if w == &word1 {
                i1 = Some(i);
            } else if w == &word2 {
                i2 = Some(i);
            }
            if let (Some(a), Some(b)) = (i1, i2) {
                best = best.min(a.abs_diff(b));
            }
        }
    }
    best as i32
}

fn main() {
    println!(
        "{}",
        shortest_distance(
            vec![
                "practice".into(),
                "makes".into(),
                "perfect".into(),
                "coding".into(),
                "makes".into(),
            ],
            "makes".into(),
            "coding".into(),
        )
    );
}

#[cfg(test)]
mod tests {
    use super::shortest_distance;

    #[test]
    fn example_one() {
        assert_eq!(
            shortest_distance(
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
            shortest_distance(
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
