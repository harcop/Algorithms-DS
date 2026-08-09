/// LeetCode #3093 - Longest Common Suffix Queries
const INF: i32 = i32::MAX;

struct TrieNode {
    children: [Option<Box<TrieNode>>; 26],
    length: i32,
    idx: i32,
}

impl TrieNode {
    fn new() -> Self {
        Self {
            children: Default::default(),
            length: INF,
            idx: INF,
        }
    }

    fn insert(&mut self, w: &[u8], i: i32) {
        let len = w.len() as i32;
        if self.length > len {
            self.length = len;
            self.idx = i;
        }
        let mut node = self;
        for &c in w.iter().rev() {
            let idx = (c - b'a') as usize;
            if node.children[idx].is_none() {
                node.children[idx] = Some(Box::new(TrieNode::new()));
            }
            node = node.children[idx].as_mut().unwrap();
            if node.length > len {
                node.length = len;
                node.idx = i;
            }
        }
    }

    fn query(&self, w: &[u8]) -> i32 {
        let mut node = self;
        for &c in w.iter().rev() {
            let idx = (c - b'a') as usize;
            match &node.children[idx] {
                Some(child) => node = child,
                None => break,
            }
        }
        node.idx
    }
}

fn string_indices(words_container: Vec<String>, words_query: Vec<String>) -> Vec<i32> {
    let mut trie = TrieNode::new();
    for (i, w) in words_container.iter().enumerate() {
        trie.insert(w.as_bytes(), i as i32);
    }
    words_query
        .iter()
        .map(|w| trie.query(w.as_bytes()))
        .collect()
}

fn main() {
    println!(
        "{:?}",
        string_indices(
            vec!["abcd".into(), "bcd".into(), "xbcd".into()],
            vec!["cd".into(), "bcd".into(), "xyz".into()]
        )
    );
}

#[cfg(test)]
mod tests {
    use super::string_indices;

    #[test]
    fn example1() {
        assert_eq!(
            string_indices(
                vec!["abcd".into(), "bcd".into(), "xbcd".into()],
                vec!["cd".into(), "bcd".into(), "xyz".into()]
            ),
            vec![1, 1, 1]
        );
    }

    #[test]
    fn example2() {
        assert_eq!(
            string_indices(
                vec!["abcdefgh".into(), "poiuygh".into(), "ghghgh".into()],
                vec!["gh".into(), "acbfgh".into(), "acbfegh".into()]
            ),
            vec![2, 0, 2]
        );
    }
}
