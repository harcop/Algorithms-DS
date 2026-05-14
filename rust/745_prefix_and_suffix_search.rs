/// LeetCode #745 - Prefix and Suffix Search
use std::collections::HashMap;

struct WordFilter {
    index: HashMap<String, i32>,
}

impl WordFilter {
    fn new(words: Vec<String>) -> Self {
        let mut index = HashMap::new();
        for (wi, w) in words.into_iter().enumerate() {
            let b = w.as_bytes();
            let n = b.len();
            for i in 0..=n {
                for j in 0..=n {
                    let pre = std::str::from_utf8(&b[..i]).unwrap();
                    let suf = std::str::from_utf8(&b[j..]).unwrap();
                    let key = format!("{}#{}", pre, suf);
                    index.insert(key, wi as i32);
                }
            }
        }
        Self { index }
    }

    fn f(&self, prefix: String, suffix: String) -> i32 {
        let key = format!("{}#{}", prefix, suffix);
        *self.index.get(&key).unwrap_or(&-1)
    }
}

fn main() {
    let wf = WordFilter::new(vec!["apple".into()]);
    println!("{}", wf.f("a".into(), "e".into()));
}

#[cfg(test)]
mod tests {
    use super::WordFilter;

    #[test]
    fn example() {
        let wf = WordFilter::new(vec![
            "apple".into(),
            "bat".into(),
            "ball".into(),
            "apricot".into(),
        ]);
        assert_eq!(wf.f("ap".into(), "e".into()), 0);
        assert!(wf.f("ba".into(), "l".into()) >= 1);
    }
}
