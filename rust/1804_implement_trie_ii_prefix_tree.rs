/// LeetCode #1804 - Implement Trie II (Prefix Tree)
struct TrieNode {
    children: [Option<Box<TrieNode>>; 26],
    word_count: i32,
    prefix_count: i32,
}

impl TrieNode {
    fn new() -> Self {
        TrieNode {
            children: std::array::from_fn(|_| None),
            word_count: 0,
            prefix_count: 0,
        }
    }
}

pub struct Trie {
    root: TrieNode,
}

impl Trie {
    fn new() -> Self {
        Trie {
            root: TrieNode::new(),
        }
    }

    fn insert(&mut self, word: String) {
        let mut node = &mut self.root;
        for c in word.bytes() {
            let idx = (c - b'a') as usize;
            if node.children[idx].is_none() {
                node.children[idx] = Some(Box::new(TrieNode::new()));
            }
            node = node.children[idx].as_mut().unwrap();
            node.prefix_count += 1;
        }
        node.word_count += 1;
    }

    fn count_words_equal_to(&self, word: String) -> i32 {
        self.find_node(word).map_or(0, |n| n.word_count)
    }

    fn count_words_starting_with(&self, prefix: String) -> i32 {
        self.find_node(prefix).map_or(0, |n| n.prefix_count)
    }

    fn erase(&mut self, word: String) {
        let mut node = &mut self.root;
        for c in word.bytes() {
            let idx = (c - b'a') as usize;
            node = node.children[idx].as_mut().unwrap();
            node.prefix_count -= 1;
        }
        node.word_count -= 1;
    }

    fn find_node(&self, s: String) -> Option<&TrieNode> {
        let mut node = &self.root;
        for c in s.bytes() {
            let idx = (c - b'a') as usize;
            node = node.children[idx].as_ref()?;
        }
        Some(node)
    }
}

fn main() {
    let mut trie = Trie::new();
    trie.insert("apple".into());
    trie.insert("apple".into());
    println!("{}", trie.count_words_equal_to("apple".into()));
}

#[cfg(test)]
mod tests {
    use super::Trie;

    #[test]
    fn example_one() {
        let mut trie = Trie::new();
        trie.insert("apple".into());
        trie.insert("apple".into());
        assert_eq!(trie.count_words_equal_to("apple".into()), 2);
        assert_eq!(trie.count_words_starting_with("app".into()), 2);
        trie.erase("apple".into());
        assert_eq!(trie.count_words_equal_to("apple".into()), 1);
        assert_eq!(trie.count_words_starting_with("app".into()), 1);
        trie.erase("apple".into());
        assert_eq!(trie.count_words_starting_with("app".into()), 0);
    }
}
