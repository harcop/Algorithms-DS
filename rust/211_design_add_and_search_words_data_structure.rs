/// LeetCode #211 - Design Add and Search Words Data Structure
use std::collections::HashMap;

#[derive(Default)]
struct Node {
    children: HashMap<char, Node>,
    end: bool,
}

pub struct WordDictionary {
    root: Node,
}

impl WordDictionary {
    fn new() -> Self {
        WordDictionary {
            root: Node::default(),
        }
    }

    fn add_word(&mut self, word: String) {
        let mut cur = &mut self.root;
        for c in word.chars() {
            cur = cur.children.entry(c).or_default();
        }
        cur.end = true;
    }

    fn search(&self, word: String) -> bool {
        fn dfs(node: &Node, bytes: &[u8], i: usize) -> bool {
            if i == bytes.len() {
                return node.end;
            }
            let ch = bytes[i] as char;
            if ch == '.' {
                for child in node.children.values() {
                    if dfs(child, bytes, i + 1) {
                        return true;
                    }
                }
                false
            } else {
                match node.children.get(&ch) {
                    Some(child) => dfs(child, bytes, i + 1),
                    None => false,
                }
            }
        }
        dfs(&self.root, word.as_bytes(), 0)
    }
}

fn main() {
    let mut d = WordDictionary::new();
    d.add_word("bad".into());
    println!("{}", d.search("b..".into()));
}

#[cfg(test)]
mod tests {
    use super::WordDictionary;

    #[test]
    fn example() {
        let mut d = WordDictionary::new();
        d.add_word("bad".into());
        d.add_word("dad".into());
        d.add_word("mad".into());
        assert!(!d.search("pad".into()));
        assert!(d.search("bad".into()));
        assert!(d.search(".ad".into()));
        assert!(d.search("b..".into()));
    }
}
