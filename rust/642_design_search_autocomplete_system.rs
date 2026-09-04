/// LeetCode #642 - Design Search Autocomplete System
use std::collections::HashMap;

struct AutocompleteSystem {
    freq: HashMap<String, i32>,
    cur: String,
}

impl AutocompleteSystem {
    fn new(sentences: Vec<String>, times: Vec<i32>) -> Self {
        let mut freq = HashMap::new();
        for (s, t) in sentences.into_iter().zip(times) {
            freq.insert(s, t);
        }
        AutocompleteSystem {
            freq,
            cur: String::new(),
        }
    }

    fn input(&mut self, c: char) -> Vec<String> {
        if c == '#' {
            *self.freq.entry(self.cur.clone()).or_insert(0) += 1;
            self.cur.clear();
            return vec![];
        }
        self.cur.push(c);
        let mut cands: Vec<(i32, String)> = self
            .freq
            .iter()
            .filter(|(s, _)| s.starts_with(&self.cur))
            .map(|(s, &t)| (-t, s.clone()))
            .collect();
        cands.sort();
        cands.into_iter().take(3).map(|(_, s)| s).collect()
    }
}

fn main() {
    let mut ac = AutocompleteSystem::new(
        vec![
            "i love you".into(),
            "island".into(),
            "ironman".into(),
            "i love leetcode".into(),
        ],
        vec![5, 3, 2, 2],
    );
    println!("{:?}", ac.input('i'));
}

#[cfg(test)]
mod tests {
    use super::AutocompleteSystem;

    #[test]
    fn example() {
        let mut ac = AutocompleteSystem::new(
            vec![
                "i love you".into(),
                "island".into(),
                "ironman".into(),
                "i love leetcode".into(),
            ],
            vec![5, 3, 2, 2],
        );
        assert_eq!(
            ac.input('i'),
            vec!["i love you", "island", "i love leetcode"]
        );
        assert_eq!(ac.input(' '), vec!["i love you", "i love leetcode"]);
        assert_eq!(ac.input('a'), Vec::<String>::new());
        assert_eq!(ac.input('#'), Vec::<String>::new());
    }
}
