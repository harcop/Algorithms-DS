/// LeetCode #1152 - Analyze User Website Visit Pattern
use std::collections::{HashMap, HashSet};

fn most_visited_pattern(
    username: Vec<String>,
    timestamp: Vec<i32>,
    website: Vec<String>,
) -> Vec<String> {
    let mut trips: Vec<(i32, String, String)> = timestamp
        .into_iter()
        .zip(username)
        .zip(website)
        .map(|((t, u), w)| (t, u, w))
        .collect();
    trips.sort_by_key(|t| t.0);
    let mut by_user: HashMap<String, Vec<String>> = HashMap::new();
    for (_, u, w) in trips {
        by_user.entry(u).or_default().push(w);
    }
    let mut score: HashMap<(String, String, String), i32> = HashMap::new();
    for sites in by_user.values() {
        if sites.len() < 3 {
            continue;
        }
        let mut patterns = HashSet::new();
        for i in 0..sites.len() {
            for j in i + 1..sites.len() {
                for k in j + 1..sites.len() {
                    patterns.insert((sites[i].clone(), sites[j].clone(), sites[k].clone()));
                }
            }
        }
        for p in patterns {
            *score.entry(p).or_insert(0) += 1;
        }
    }
    let mut best: Option<(&(String, String, String), &i32)> = None;
    for (pat, sc) in &score {
        match best {
            None => best = Some((pat, sc)),
            Some((bp, bs)) => {
                if sc > bs || (sc == bs && pat < bp) {
                    best = Some((pat, sc));
                }
            }
        }
    }
    let (a, b, c) = best.unwrap().0.clone();
    vec![a, b, c]
}

fn main() {
    let username = vec![
        "joe".into(),
        "joe".into(),
        "joe".into(),
        "james".into(),
        "james".into(),
        "james".into(),
        "james".into(),
        "mary".into(),
        "mary".into(),
        "mary".into(),
    ];
    let timestamp = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let website = vec![
        "home".into(),
        "about".into(),
        "career".into(),
        "home".into(),
        "cart".into(),
        "maps".into(),
        "home".into(),
        "home".into(),
        "about".into(),
        "career".into(),
    ];
    println!("{:?}", most_visited_pattern(username, timestamp, website));
}

#[cfg(test)]
mod tests {
    use super::most_visited_pattern;

    #[test]
    fn example_one() {
        let username = vec![
            "joe".into(),
            "joe".into(),
            "joe".into(),
            "james".into(),
            "james".into(),
            "james".into(),
            "james".into(),
            "mary".into(),
            "mary".into(),
            "mary".into(),
        ];
        let timestamp = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        let website = vec![
            "home".into(),
            "about".into(),
            "career".into(),
            "home".into(),
            "cart".into(),
            "maps".into(),
            "home".into(),
            "home".into(),
            "about".into(),
            "career".into(),
        ];
        assert_eq!(
            most_visited_pattern(username, timestamp, website),
            vec!["home".to_string(), "about".to_string(), "career".to_string()]
        );
    }

    #[test]
    fn example_two() {
        let username = vec![
            "ua".into(),
            "ua".into(),
            "ua".into(),
            "ub".into(),
            "ub".into(),
            "ub".into(),
        ];
        let timestamp = vec![1, 2, 3, 4, 5, 6];
        let website = vec![
            "a".into(),
            "b".into(),
            "a".into(),
            "a".into(),
            "b".into(),
            "c".into(),
        ];
        assert_eq!(
            most_visited_pattern(username, timestamp, website),
            vec!["a".to_string(), "b".to_string(), "a".to_string()]
        );
    }
}
