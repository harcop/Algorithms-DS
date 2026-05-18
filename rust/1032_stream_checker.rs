/// LeetCode #1032 - Stream Checker
use std::collections::HashMap;

struct StreamChecker {
    trie: Vec<HashMap<char, usize>>,
    ends: Vec<bool>,
    rev_buf: Vec<char>,
    max_len: usize,
}

impl StreamChecker {
    fn new(words: Vec<String>) -> Self {
        let mut trie = vec![HashMap::new()];
        let mut ends = vec![false];
        let mut max_len = 0usize;
        for w in words {
            max_len = max_len.max(w.len());
            let rev: Vec<char> = w.chars().rev().collect();
            let mut node = 0usize;
            for c in rev {
                let next = *trie[node].entry(c).or_insert_with(|| {
                    ends.push(false);
                    trie.push(HashMap::new());
                    trie.len() - 1
                });
                node = next;
            }
            ends[node] = true;
        }
        StreamChecker {
            trie,
            ends,
            rev_buf: Vec::new(),
            max_len,
        }
    }

    fn query(&mut self, letter: char) -> bool {
        self.rev_buf.push(letter);
        if self.rev_buf.len() > self.max_len {
            self.rev_buf.remove(0);
        }
        let mut node = 0usize;
        for &c in self.rev_buf.iter().rev() {
            let Some(&next) = self.trie[node].get(&c) else {
                return false;
            };
            node = next;
            if self.ends[node] {
                return true;
            }
        }
        false
    }
}

fn main() {
    let mut sc = StreamChecker::new(vec!["cd".into(), "f".into(), "kl".into()]);
    println!("{}", sc.query('a'));
}

#[cfg(test)]
mod tests {
    use super::StreamChecker;

    #[test]
    fn example_one() {
        let mut sc = StreamChecker::new(vec!["cd".into(), "f".into(), "kl".into()]);
        assert!(!sc.query('a'));
        assert!(!sc.query('b'));
        assert!(!sc.query('c'));
        assert!(sc.query('d'));
    }
}
