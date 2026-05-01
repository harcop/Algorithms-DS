use std::collections::{HashMap, HashSet};

/// LeetCode #126 - Word Ladder II
fn find_ladders(begin_word: String, end_word: String, word_list: Vec<String>) -> Vec<Vec<String>> {
    let mut dict: HashSet<String> = word_list.into_iter().collect();
    if !dict.contains(&end_word) {
        return vec![];
    }

    let mut res: Vec<Vec<String>> = vec![];
    let mut layer: HashMap<String, Vec<Vec<String>>> = HashMap::new();
    layer.insert(begin_word.clone(), vec![vec![begin_word.clone()]]);

    while !layer.is_empty() {
        let mut next_layer: HashMap<String, Vec<Vec<String>>> = HashMap::new();
        for w in layer.keys() {
            dict.remove(w);
        }
        for (word, paths) in &layer {
            let bytes = word.as_bytes().to_vec();
            for i in 0..bytes.len() {
                let mut v = bytes.clone();
                for c in b'a'..=b'z' {
                    if c == bytes[i] {
                        continue;
                    }
                    v[i] = c;
                    let nw = String::from_utf8(v.clone()).unwrap();
                    if dict.contains(&nw) {
                        for p in paths {
                            let mut np = p.clone();
                            np.push(nw.clone());
                            next_layer.entry(nw.clone()).or_default().push(np);
                        }
                    }
                }
            }
        }
        for (w, ps) in &next_layer {
            if w == &end_word {
                res.extend(ps.clone());
            }
        }
        if !res.is_empty() {
            return res;
        }
        layer = next_layer;
    }
    res
}

fn main() {
    println!(
        "{:?}",
        find_ladders(
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
    use super::find_ladders;

    fn normalize(mut v: Vec<Vec<String>>) -> Vec<Vec<String>> {
        v.sort();
        v
    }

    #[test]
    fn example_one() {
        let got = normalize(find_ladders(
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
        ));
        let mut expected = normalize(vec![
            vec!["hit", "hot", "dot", "dog", "cog"],
            vec!["hit", "hot", "lot", "log", "cog"],
        ]
        .into_iter()
        .map(|p| p.into_iter().map(String::from).collect())
        .collect::<Vec<_>>());
        expected.sort();
        assert_eq!(got, expected);
    }

    #[test]
    fn example_two() {
        assert!(find_ladders(
            "hit".to_string(),
            "cog".to_string(),
            vec![
                "hot".to_string(),
                "dot".to_string(),
                "dog".to_string(),
                "lot".to_string(),
                "log".to_string(),
            ],
        )
        .is_empty());
    }
}
