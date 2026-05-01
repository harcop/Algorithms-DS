use std::collections::{HashSet, VecDeque};

/// LeetCode #127 - Word Ladder
fn ladder_length(begin_word: String, end_word: String, word_list: Vec<String>) -> i32 {
    let dict: HashSet<String> = word_list.into_iter().collect();
    if !dict.contains(&end_word) {
        return 0;
    }
    let mut q = VecDeque::new();
    q.push_back((begin_word.clone(), 1));
    let mut seen = HashSet::new();
    seen.insert(begin_word);

    while let Some((w, d)) = q.pop_front() {
        if w == end_word {
            return d;
        }
        let bytes = w.as_bytes().to_vec();
        for i in 0..bytes.len() {
            let mut v = bytes.clone();
            for c in b'a'..=b'z' {
                if c == bytes[i] {
                    continue;
                }
                v[i] = c;
                let nw = String::from_utf8(v.clone()).unwrap();
                if dict.contains(&nw) && seen.insert(nw.clone()) {
                    q.push_back((nw, d + 1));
                }
            }
        }
    }
    0
}

fn main() {
    println!(
        "{}",
        ladder_length(
            "hit".to_string(),
            "cog".to_string(),
            vec![
                "hot".to_string(),
                "dot".to_string(),
                "dog".to_string(),
                "lot".to_string(),
                "log".to_string(),
                "cog".to_string(),
            ],
        )
    );
}

#[cfg(test)]
mod tests {
    use super::ladder_length;

    #[test]
    fn example_one() {
        assert_eq!(
            ladder_length(
                "hit".to_string(),
                "cog".to_string(),
                vec![
                    "hot".to_string(),
                    "dot".to_string(),
                    "dog".to_string(),
                    "lot".to_string(),
                    "log".to_string(),
                    "cog".to_string(),
                ],
            ),
            5
        );
    }

    #[test]
    fn example_two() {
        assert_eq!(
            ladder_length(
                "hit".to_string(),
                "cog".to_string(),
                vec![
                    "hot".to_string(),
                    "dot".to_string(),
                    "dog".to_string(),
                    "lot".to_string(),
                    "log".to_string(),
                ],
            ),
            0
        );
    }
}
