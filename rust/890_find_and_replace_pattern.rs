/// LeetCode #890 - Find and Replace Pattern
use std::collections::HashMap;

fn match_pattern(word: &str, pattern: &str) -> bool {
    let w: Vec<char> = word.chars().collect();
    let p: Vec<char> = pattern.chars().collect();
    if w.len() != p.len() {
        return false;
    }
    let mut wp = HashMap::new();
    let mut pw = HashMap::new();
    for i in 0..w.len() {
        if let Some(&cw) = wp.get(&p[i]) {
            if cw != w[i] {
                return false;
            }
        } else {
            wp.insert(p[i], w[i]);
        }
        if let Some(&cp) = pw.get(&w[i]) {
            if cp != p[i] {
                return false;
            }
        } else {
            pw.insert(w[i], p[i]);
        }
    }
    true
}

fn find_and_replace_pattern(words: Vec<String>, pattern: String) -> Vec<String> {
    let ps = pattern.as_str();
    words
        .into_iter()
        .filter(|w| match_pattern(w.as_str(), ps))
        .collect()
}

fn main() {
    println!(
        "{:?}",
        find_and_replace_pattern(
            vec!["abc", "deq", "mee", "aqq", "dkd", "ccc"]
                .into_iter()
                .map(String::from)
                .collect(),
            "abb".into()
        )
    );
}

#[cfg(test)]
mod tests {
    use super::find_and_replace_pattern;

    #[test]
    fn example_one() {
        let w: Vec<String> = vec!["abc", "deq", "mee", "aqq", "dkd", "ccc"]
            .into_iter()
            .map(String::from)
            .collect();
        let got = find_and_replace_pattern(w, "abb".into());
        assert_eq!(got, vec!["mee", "aqq"]);
    }
}
