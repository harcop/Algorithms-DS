/// LeetCode #966 - Vowel Spellchecker
use std::collections::HashMap;

fn is_vowel(c: u8) -> bool {
    matches!(c, b'a' | b'e' | b'i' | b'o' | b'u')
}

fn devowel(s: &str) -> String {
    s.bytes()
        .map(|b| {
            let c = b.to_ascii_lowercase();
            if is_vowel(c) {
                '#'
            } else {
                c as char
            }
        })
        .collect()
}

fn spellchecker(wordlist: Vec<String>, queries: Vec<String>) -> Vec<String> {
    let mut exact: HashMap<&str, &str> = HashMap::new();
    let mut lower: HashMap<String, &str> = HashMap::new();
    let mut vowel: HashMap<String, &str> = HashMap::new();
    let mut vowel_cnt: HashMap<String, usize> = HashMap::new();

    for w in &wordlist {
        let ws = w.as_str();
        exact.entry(ws).or_insert(ws);
        let lw = ws.to_ascii_lowercase();
        lower.entry(lw.clone()).or_insert(ws);
        let key = devowel(ws);
        *vowel_cnt.entry(key.clone()).or_insert(0) += 1;
        vowel.entry(key).or_insert(ws);
    }

    queries
        .into_iter()
        .map(|q| {
            if let Some(&w) = exact.get(q.as_str()) {
                return w.to_string();
            }
            if q.chars().any(|c| c.is_ascii_lowercase()) {
                if let Some(&w) = lower.get(&q.to_ascii_lowercase()) {
                    return w.to_string();
                }
                let key = devowel(&q);
                if let Some(&cnt) = vowel_cnt.get(&key) {
                    if cnt == 1 {
                        return vowel[&key].to_string();
                    }
                    if q.ends_with('o') {
                        return String::new();
                    }
                    return vowel[&key].to_string();
                }
            }
            String::new()
        })
        .collect()
}

fn main() {
    let wl = vec!["KiTe".into(), "kite".into(), "hare".into(), "Hare".into()];
    println!("{:?}", spellchecker(wl, vec!["kite".into(), "Kite".into()]));
}

#[cfg(test)]
mod tests {
    use super::spellchecker;

    #[test]
    fn example() {
        let wl = vec![
            "KiTe".into(),
            "kite".into(),
            "hare".into(),
            "Hare".into(),
        ];
        let out = spellchecker(
            wl,
            vec![
                "kite".into(),
                "Kite".into(),
                "KiTe".into(),
                "Hare".into(),
                "hare".into(),
                "HARE".into(),
                "HEAR".into(),
                "hear".into(),
                "keti".into(),
                "keet".into(),
                "keto".into(),
            ],
        );
        assert_eq!(
            out,
            vec![
                "kite", "KiTe", "KiTe", "Hare", "hare", "", "", "", "KiTe", "", ""
            ]
        );
    }
}
