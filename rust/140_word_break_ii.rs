use std::collections::HashMap;

/// LeetCode #140 - Word Break II
fn word_break(s: String, word_dict: Vec<String>) -> Vec<String> {
    let dict: Vec<String> = word_dict;
    let mut memo: HashMap<usize, Vec<String>> = HashMap::new();

    fn dfs(
        s: &str,
        dict: &[String],
        start: usize,
        memo: &mut HashMap<usize, Vec<String>>,
    ) -> Vec<String> {
        if let Some(v) = memo.get(&start) {
            return v.clone();
        }
        let mut res = Vec::new();
        if start == s.len() {
            res.push(String::new());
            return res;
        }
        for w in dict {
            if s[start..].starts_with(w) {
                let sub = dfs(s, dict, start + w.len(), memo);
                for mut t in sub {
                    if t.is_empty() {
                        res.push(w.clone());
                    } else {
                        t = format!("{w} {t}");
                        res.push(t);
                    }
                }
            }
        }
        memo.insert(start, res.clone());
        res
    }

    dfs(&s, &dict, 0, &mut memo)
        .into_iter()
        .filter(|x| !x.is_empty())
        .collect()
}

fn main() {
    println!(
        "{:?}",
        word_break(
            "catsanddog".to_string(),
            vec![
                "cat".to_string(),
                "cats".to_string(),
                "and".to_string(),
                "sand".to_string(),
                "dog".to_string(),
            ],
        )
    );
}

#[cfg(test)]
mod tests {
    use super::word_break;

    fn normalize(mut v: Vec<String>) -> Vec<String> {
        v.sort();
        v
    }

    #[test]
    fn example_one() {
        let got = normalize(word_break(
            "catsanddog".to_string(),
            vec![
                "cat".to_string(),
                "cats".to_string(),
                "and".to_string(),
                "sand".to_string(),
                "dog".to_string(),
            ],
        ));
        let expected = normalize(vec![
            "cats and dog".to_string(),
            "cat sand dog".to_string(),
        ]);
        assert_eq!(got, expected);
    }

    #[test]
    fn example_two() {
        let got = normalize(word_break(
            "pineapplepenapple".to_string(),
            vec![
                "apple".to_string(),
                "pen".to_string(),
                "applepen".to_string(),
                "pine".to_string(),
                "pineapple".to_string(),
            ],
        ));
        assert_eq!(got.len(), 3);
    }
}
