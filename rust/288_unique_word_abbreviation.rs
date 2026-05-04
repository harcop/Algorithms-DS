/// LeetCode #288 - Unique Word Abbreviation
use std::collections::HashMap;

fn abbr(w: &str) -> String {
    let b = w.as_bytes();
    if b.len() <= 2 {
        return w.to_string();
    }
    format!(
        "{}{}{}",
        b[0] as char,
        b.len() - 2,
        b[b.len() - 1] as char
    )
}

pub struct ValidWordAbbr {
    map: HashMap<String, Vec<String>>,
}

impl ValidWordAbbr {
    fn new(dictionary: Vec<String>) -> Self {
        let mut map: HashMap<String, Vec<String>> = HashMap::new();
        for w in dictionary {
            let a = abbr(&w);
            map.entry(a).or_default().push(w);
        }
        ValidWordAbbr { map }
    }

    fn is_unique(&self, word: String) -> bool {
        let a = abbr(&word);
        match self.map.get(&a) {
            None => true,
            Some(v) => v.len() == 1 && v[0] == word,
        }
    }
}

fn main() {
    let v = ValidWordAbbr::new(vec!["deer".into(), "door".into(), "cake".into(), "card".into()]);
    println!("{}", v.is_unique("dear".into()));
}

#[cfg(test)]
mod tests {
    use super::ValidWordAbbr;

    #[test]
    fn example() {
        let v = ValidWordAbbr::new(vec!["deer".into(), "door".into(), "cake".into(), "card".into()]);
        assert!(!v.is_unique("dear".into()));
        assert!(v.is_unique("cart".into()));
        assert!(!v.is_unique("cane".into()));
        assert!(v.is_unique("make".into()));
    }
}
