/// LeetCode #1772 - Sort Features by Popularity
use std::collections::{HashMap, HashSet};

fn sort_features(features: Vec<String>, responses: Vec<String>) -> Vec<String> {
    let mut idx: HashMap<&str, usize> = HashMap::new();
    for (i, f) in features.iter().enumerate() {
        idx.insert(f.as_str(), i);
    }
    let mut score = vec![0i32; features.len()];
    for resp in &responses {
        let words: HashSet<&str> = resp.split_whitespace().collect();
        for w in words {
            if let Some(&i) = idx.get(w) {
                score[i] += 1;
            }
        }
    }
    let mut order: Vec<usize> = (0..features.len()).collect();
    order.sort_by(|&a, &b| score[b].cmp(&score[a]).then(a.cmp(&b)));
    order.into_iter().map(|i| features[i].clone()).collect()
}

fn main() {
    let features = vec!["cooler".into(), "lock".into(), "touch".into()];
    let responses = vec![
        "i like cooler cooler".into(),
        "lock touch cool".into(),
        "locker like touch".into(),
    ];
    println!("{:?}", sort_features(features, responses));
}

#[cfg(test)]
mod tests {
    use super::sort_features;

    #[test]
    fn example_one() {
        let features = vec!["cooler".into(), "lock".into(), "touch".into()];
        let responses = vec![
            "i like cooler cooler".into(),
            "lock touch cool".into(),
            "locker like touch".into(),
        ];
        assert_eq!(
            sort_features(features, responses),
            vec!["touch".to_string(), "cooler".to_string(), "lock".to_string()]
        );
    }

    #[test]
    fn example_two() {
        let features = vec!["a".into(), "aa".into(), "b".into(), "c".into()];
        let responses = vec![
            "a".into(),
            "a aa".into(),
            "a a a a a".into(),
            "b a".into(),
        ];
        assert_eq!(
            sort_features(features, responses),
            vec![
                "a".to_string(),
                "aa".to_string(),
                "b".to_string(),
                "c".to_string()
            ]
        );
    }
}
