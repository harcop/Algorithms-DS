/// LeetCode #290 - Word Pattern
use std::collections::HashMap;

fn word_pattern(pattern: String, s: String) -> bool {
    let words: Vec<&str> = s.split_whitespace().collect();
    if pattern.len() != words.len() {
        return false;
    }
    let mut p2w: HashMap<char, &str> = HashMap::new();
    let mut w2p: HashMap<&str, char> = HashMap::new();
    for (c, w) in pattern.chars().zip(words.into_iter()) {
        if let Some(&ew) = p2w.get(&c) {
            if ew != w {
                return false;
            }
        } else {
            p2w.insert(c, w);
        }
        if let Some(&ec) = w2p.get(&w) {
            if ec != c {
                return false;
            }
        } else {
            w2p.insert(w, c);
        }
    }
    true
}

fn main() {
    println!("{}", word_pattern("abba".into(), "dog cat cat dog".into()));
}

#[cfg(test)]
mod tests {
    use super::word_pattern;

    #[test]
    fn example_one() {
        assert!(word_pattern("abba".into(), "dog cat cat dog".into()));
    }

    #[test]
    fn example_two() {
        assert!(!word_pattern("abba".into(), "dog cat cat fish".into()));
    }
}
