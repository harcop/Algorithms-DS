/// LeetCode #692 - Top K Frequent Words
use std::collections::HashMap;

fn top_k_frequent(words: Vec<String>, k: i32) -> Vec<String> {
    let mut count: HashMap<String, i32> = HashMap::new();
    for w in words {
        *count.entry(w).or_insert(0) += 1;
    }
    let mut entries: Vec<(String, i32)> = count.into_iter().collect();
    entries.sort_unstable_by(|a, b| b.1.cmp(&a.1).then(a.0.cmp(&b.0)));
    entries
        .into_iter()
        .take(k as usize)
        .map(|(s, _)| s)
        .collect()
}

fn main() {
    println!(
        "{:?}",
        top_k_frequent(
            vec!["i".into(), "love".into(), "leetcode".into(), "i".into(), "love".into(), "coding".into()],
            2
        )
    );
}

#[cfg(test)]
mod tests {
    use super::top_k_frequent;

    #[test]
    fn example_one() {
        assert_eq!(
            top_k_frequent(
                vec!["i".into(), "love".into(), "leetcode".into(), "i".into(), "love".into(), "coding".into()],
                2
            ),
            vec!["i".to_string(), "love".to_string()]
        );
    }

    #[test]
    fn example_two() {
        assert_eq!(
            top_k_frequent(
                vec![
                    "the".into(),
                    "day".into(),
                    "is".into(),
                    "sunny".into(),
                    "the".into(),
                    "the".into(),
                    "the".into(),
                    "sunny".into(),
                    "is".into(),
                    "is".into()
                ],
                4
            ),
            vec![
                "the".to_string(),
                "is".to_string(),
                "sunny".to_string(),
                "day".to_string()
            ]
        );
    }
}
