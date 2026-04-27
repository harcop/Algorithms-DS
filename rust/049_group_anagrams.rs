use std::collections::HashMap;

/// LeetCode #49 - Group Anagrams
fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
    let mut groups: HashMap<[u8; 26], Vec<String>> = HashMap::new();

    for s in strs {
        let mut key = [0u8; 26];
        for &b in s.as_bytes() {
            key[(b - b'a') as usize] += 1;
        }
        groups.entry(key).or_default().push(s);
    }

    groups.into_values().collect()
}

fn main() {
    println!(
        "{:?}",
        group_anagrams(vec![
            "eat".to_string(),
            "tea".to_string(),
            "tan".to_string(),
            "ate".to_string(),
            "nat".to_string(),
            "bat".to_string()
        ])
    );
}

#[cfg(test)]
mod tests {
    use super::group_anagrams;

    fn normalize(mut groups: Vec<Vec<String>>) -> Vec<Vec<String>> {
        for g in &mut groups {
            g.sort();
        }
        groups.sort();
        groups
    }

    #[test]
    fn example_one() {
        let got = normalize(group_anagrams(vec![
            "eat".to_string(),
            "tea".to_string(),
            "tan".to_string(),
            "ate".to_string(),
            "nat".to_string(),
            "bat".to_string(),
        ]));
        let expected = normalize(vec![
            vec!["bat".to_string()],
            vec!["nat".to_string(), "tan".to_string()],
            vec!["ate".to_string(), "eat".to_string(), "tea".to_string()],
        ]);
        assert_eq!(got, expected);
    }

    #[test]
    fn example_two() {
        assert_eq!(group_anagrams(vec!["".to_string()]), vec![vec!["".to_string()]]);
    }
}
