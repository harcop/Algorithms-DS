/// LeetCode #1858 - Longest Word With All Prefixes
struct TrieNode {
    children: [Option<Box<TrieNode>>; 26],
    is_end: bool,
}

impl TrieNode {
    fn new() -> Self {
        TrieNode {
            children: std::array::from_fn(|_| None),
            is_end: false,
        }
    }
}

struct Trie {
    root: TrieNode,
}

impl Trie {
    fn new() -> Self {
        Trie {
            root: TrieNode::new(),
        }
    }

    fn insert(&mut self, word: &str) {
        let mut node = &mut self.root;
        for c in word.bytes() {
            let idx = (c - b'a') as usize;
            if node.children[idx].is_none() {
                node.children[idx] = Some(Box::new(TrieNode::new()));
            }
            node = node.children[idx].as_mut().unwrap();
        }
        node.is_end = true;
    }

    fn has_all_prefixes(&self, word: &str) -> bool {
        let mut node = &self.root;
        for c in word.bytes() {
            let idx = (c - b'a') as usize;
            node = match &node.children[idx] {
                Some(n) => n.as_ref(),
                None => return false,
            };
            if !node.is_end {
                return false;
            }
        }
        true
    }
}

fn longest_word(words: Vec<String>) -> String {
    let mut trie = Trie::new();
    for w in &words {
        trie.insert(w);
    }
    let mut ans = String::new();
    for w in words {
        if trie.has_all_prefixes(&w)
            && (w.len() > ans.len() || (w.len() == ans.len() && w < ans))
        {
            ans = w;
        }
    }
    ans
}

fn main() {
    println!(
        "{}",
        longest_word(vec![
            "k".into(),
            "ki".into(),
            "kir".into(),
            "kira".into(),
            "kiran".into(),
        ])
    );
}

#[cfg(test)]
mod tests {
    use super::longest_word;

    #[test]
    fn example_one() {
        assert_eq!(
            longest_word(vec![
                "k".into(),
                "ki".into(),
                "kir".into(),
                "kira".into(),
                "kiran".into(),
            ]),
            "kiran"
        );
    }

    #[test]
    fn example_two() {
        assert_eq!(
            longest_word(vec!["a".into(), "banana".into(), "app".into(), "appl".into(), "ap".into(), "apply".into(), "apple".into()]),
            "apple"
        );
    }
}
