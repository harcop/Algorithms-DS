/// LeetCode #676 - Implement Magic Dictionary
struct MagicDictionary {
    by_len: std::collections::HashMap<usize, Vec<Vec<u8>>>,
}

impl MagicDictionary {
    fn new() -> Self {
        Self {
            by_len: std::collections::HashMap::new(),
        }
    }

    fn build_dict(&mut self, dictionary: Vec<String>) {
        for w in dictionary {
            let bytes = w.into_bytes();
            self.by_len.entry(bytes.len()).or_default().push(bytes);
        }
    }

    fn search(&self, search_word: String) -> bool {
        let needle = search_word.as_bytes();
        let Some(bucket) = self.by_len.get(&needle.len()) else {
            return false;
        };
        for w in bucket {
            let mut diff = 0;
            for i in 0..w.len() {
                if w[i] != needle[i] {
                    diff += 1;
                    if diff > 1 {
                        break;
                    }
                }
            }
            if diff == 1 {
                return true;
            }
        }
        false
    }
}

fn main() {
    let mut d = MagicDictionary::new();
    d.build_dict(vec!["hello".into(), "leetcode".into()]);
    println!("{}", d.search("hhllo".into()));
}

#[cfg(test)]
mod tests {
    use super::MagicDictionary;

    #[test]
    fn example() {
        let mut d = MagicDictionary::new();
        d.build_dict(vec![
            "hello".into(),
            "leetcode".into(),
        ]);
        assert!(!d.search("hello".into()));
        assert!(d.search("hhllo".into()));
        assert!(!d.search("hell".into()));
        assert!(!d.search("leetcoded".into()));
    }
}
