/// LeetCode #208 - Implement Trie (Prefix Tree)
use std::collections::HashMap;

#[derive(Default)]
struct TrieNode {
    children: HashMap<char, TrieNode>,
    end: bool,
}

pub struct Trie {
    root: TrieNode,
}

impl Trie {
    fn new() -> Self {
        Trie {
            root: TrieNode::default(),
        }
    }

    fn insert(&mut self, word: String) {
        let mut cur = &mut self.root;
        for c in word.chars() {
            cur = cur.children.entry(c).or_default();
        }
        cur.end = true;
    }

    fn search(&self, word: String) -> bool {
        self.walk(word.chars(), true)
    }

    fn starts_with(&self, prefix: String) -> bool {
        self.walk(prefix.chars(), false)
    }

    fn walk(&self, it: impl Iterator<Item = char>, need_end: bool) -> bool {
        let mut cur = &self.root;
        for c in it {
            match cur.children.get(&c) {
                Some(n) => cur = n,
                None => return false,
            }
        }
        !need_end || cur.end
    }
}

fn main() {
    let mut t = Trie::new();
    t.insert("apple".into());
    println!("{}", t.search("apple".into()));
}

#[cfg(test)]
mod tests {
    use super::Trie;

    #[test]
    fn example() {
        let mut t = Trie::new();
        t.insert("apple".into());
        assert!(t.search("apple".into()));
        assert!(!t.search("app".into()));
        assert!(t.starts_with("app".into()));
        t.insert("app".into());
        assert!(t.search("app".into()));
    }
}
