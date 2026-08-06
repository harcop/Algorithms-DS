/// LeetCode #3045 - Count Prefix and Suffix Pairs II
use std::collections::HashMap;

struct TrieNode {
    children: HashMap<(char, char), Box<TrieNode>>,
    cnt: i64,
}

impl TrieNode {
    fn new() -> Self {
        Self {
            children: HashMap::new(),
            cnt: 0,
        }
    }
}

fn count_prefix_suffix_pairs(words: Vec<String>) -> i64 {
    let mut root = TrieNode::new();
    let mut ans = 0i64;

    for s in words {
        let chars: Vec<char> = s.chars().collect();
        let rev: Vec<char> = s.chars().rev().collect();
        let mut node = &mut root;

        for i in 0..chars.len() {
            let key = (chars[i], rev[i]);
            node = node
                .children
                .entry(key)
                .or_insert_with(|| Box::new(TrieNode::new()));
            ans += node.cnt;
        }
        node.cnt += 1;
    }

    ans
}

fn main() {
    println!(
        "{}",
        count_prefix_suffix_pairs(vec![
            "a".into(),
            "aba".into(),
            "ababa".into(),
            "aa".into(),
        ])
    );
}

#[cfg(test)]
mod tests {
    use super::count_prefix_suffix_pairs;

    #[test]
    fn example1() {
        assert_eq!(
            count_prefix_suffix_pairs(vec![
                "a".into(),
                "aba".into(),
                "ababa".into(),
                "aa".into(),
            ]),
            4
        );
    }

    #[test]
    fn example2() {
        assert_eq!(
            count_prefix_suffix_pairs(vec!["pa".into(), "papa".into(), "ma".into(), "mama".into()]),
            2
        );
    }

    #[test]
    fn example3() {
        assert_eq!(
            count_prefix_suffix_pairs(vec!["abab".into(), "ab".into()]),
            0
        );
    }
}
