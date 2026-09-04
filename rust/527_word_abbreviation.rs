/// LeetCode #527 - Word Abbreviation
use std::collections::HashMap;

fn abbrev(word: &str, prefix_len: usize) -> String {
    let n = word.len();
    if prefix_len >= n - 1 {
        return word.to_string();
    }
    let skipped = n - prefix_len - 1;
    let abbr = format!("{}{}{}", &word[..prefix_len], skipped, &word[n - 1..]);
    if abbr.len() >= n {
        word.to_string()
    } else {
        abbr
    }
}

fn words_abbreviation(words: Vec<String>) -> Vec<String> {
    let n = words.len();
    let mut prefix = vec![1usize; n];
    let mut ans = vec![String::new(); n];
    let mut resolved = vec![false; n];
    let mut remaining = n;
    while remaining > 0 {
        let mut groups: HashMap<String, Vec<usize>> = HashMap::new();
        for i in 0..n {
            if resolved[i] {
                continue;
            }
            let a = abbrev(&words[i], prefix[i]);
            groups.entry(a).or_default().push(i);
        }
        for (a, idxs) in groups {
            if idxs.len() == 1 {
                let i = idxs[0];
                ans[i] = a;
                resolved[i] = true;
                remaining -= 1;
            } else {
                for i in idxs {
                    prefix[i] += 1;
                }
            }
        }
    }
    ans
}

fn main() {
    let words = vec!["like".into(), "god".into(), "internal".into()];
    println!("{:?}", words_abbreviation(words));
}

#[cfg(test)]
mod tests {
    use super::words_abbreviation;

    #[test]
    fn example() {
        let words = vec![
            "like".into(),
            "god".into(),
            "internal".into(),
            "me".into(),
            "internet".into(),
            "interval".into(),
            "intension".into(),
            "face".into(),
            "intrusion".into(),
        ];
        assert_eq!(
            words_abbreviation(words),
            vec![
                "l2e",
                "god",
                "internal",
                "me",
                "i6t",
                "interval",
                "inte4n",
                "f2e",
                "intr4n",
            ]
        );
    }
}
