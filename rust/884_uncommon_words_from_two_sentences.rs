/// LeetCode #884 - Uncommon Words from Two Sentences
use std::collections::HashMap;

fn uncommon_from_sentences(s1: String, s2: String) -> Vec<String> {
    let mut cnt: HashMap<&str, i32> = HashMap::new();
    for w in s1.split_whitespace() {
        *cnt.entry(w).or_insert(0) += 1;
    }
    for w in s2.split_whitespace() {
        *cnt.entry(w).or_insert(0) += 1;
    }
    cnt.into_iter()
        .filter(|&(_, c)| c == 1)
        .map(|(w, _)| w.to_string())
        .collect()
}

fn main() {
    println!(
        "{:?}",
        uncommon_from_sentences("this apple is sweet".to_string(), "this apple is sour".to_string())
    );
}

#[cfg(test)]
mod tests {
    use super::uncommon_from_sentences;

    #[test]
    fn example_one() {
        let mut got = uncommon_from_sentences(
            "this apple is sweet".to_string(),
            "this apple is sour".to_string(),
        );
        got.sort();
        assert_eq!(got, vec!["sour", "sweet"]);
    }
}
