/// LeetCode #472 - Concatenated Words
use std::collections::HashSet;

fn find_all_concatenated_words_in_a_dict(words: Vec<String>) -> Vec<String> {
    let set: HashSet<&str> = words.iter().map(|s| s.as_str()).collect();
    let mut out = vec![];

    fn is_concat(word: &[u8], set: &HashSet<&str>, start: usize, tried_root: bool) -> bool {
        if start == word.len() {
            return tried_root;
        }
        for end in start + 1..=word.len() {
            let part = std::str::from_utf8(&word[start..end]).unwrap();
            if !set.contains(part) {
                continue;
            }
            if start == 0 && end == word.len() {
                continue;
            }
            if is_concat(word, set, end, true) {
                return true;
            }
        }
        false
    }

    for w in &words {
        if w.is_empty() {
            continue;
        }
        let b = w.as_bytes();
        if is_concat(b, &set, 0, false) {
            out.push(w.clone());
        }
    }
    out
}

fn main() {
    println!(
        "{:?}",
        find_all_concatenated_words_in_a_dict(vec![
            "cat".into(),
            "cats".into(),
            "catsdogcats".into(),
            "dog".into(),
            "dogcatsdog".into(),
            "hippopotamuses".into(),
            "rat".into(),
            "ratcatdogcat".into(),
        ])
    );
}

#[cfg(test)]
mod tests {
    use super::find_all_concatenated_words_in_a_dict;

    #[test]
    fn example_one() {
        let mut v = find_all_concatenated_words_in_a_dict(vec![
            "cat".into(),
            "cats".into(),
            "catsdogcats".into(),
            "dog".into(),
            "dogcatsdog".into(),
            "hippopotamuses".into(),
            "rat".into(),
            "ratcatdogcat".into(),
        ]);
        v.sort();
        let mut e: Vec<String> = vec![
            "catsdogcats".into(),
            "dogcatsdog".into(),
            "ratcatdogcat".into(),
        ];
        e.sort();
        assert_eq!(v, e);
    }
}
