/// LeetCode #819 - Most Common Word
use std::collections::HashMap;

fn most_common_word(paragraph: String, banned: Vec<String>) -> String {
    let ban: std::collections::HashSet<String> = banned.into_iter().collect();
    let mut cnt: HashMap<String, i32> = HashMap::new();
    let mut cur = String::new();
    for c in paragraph.chars().chain(std::iter::once(' ')) {
        if c.is_ascii_alphabetic() {
            cur.push(c.to_ascii_lowercase());
        } else if !cur.is_empty() {
            if !ban.contains(&cur) {
                *cnt.entry(cur.clone()).or_insert(0) += 1;
            }
            cur.clear();
        }
    }
    cnt.into_iter()
        .max_by_key(|&(_, c)| c)
        .map(|(w, _)| w)
        .unwrap()
}

fn main() {
    println!(
        "{}",
        most_common_word(
            "Bob hit a ball, the hit BALL flew far after it was hit.".into(),
            vec!["hit".into()],
        )
    );
}

#[cfg(test)]
mod tests {
    use super::most_common_word;

    #[test]
    fn example_one() {
        assert_eq!(
            most_common_word(
                "Bob hit a ball, the hit BALL flew far after it was hit.".into(),
                vec!["hit".into()],
            ),
            "ball"
        );
    }
}
