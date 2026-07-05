/// LeetCode #2227 - Encrypt and Decrypt Strings
use std::collections::HashMap;

struct Encrypter {
    mp: HashMap<char, String>,
    cnt: HashMap<String, i32>,
}

impl Encrypter {
    fn new(keys: Vec<char>, values: Vec<String>, dictionary: Vec<String>) -> Self {
        let mp: HashMap<char, String> = keys.into_iter().zip(values).collect();
        let mut cnt = HashMap::new();
        for word in dictionary {
            let encrypted = Self::encrypt_word(&mp, &word);
            if !encrypted.is_empty() {
                *cnt.entry(encrypted).or_insert(0) += 1;
            }
        }
        Self { mp, cnt }
    }

    fn encrypt_word(mp: &HashMap<char, String>, word: &str) -> String {
        let mut res = String::new();
        for c in word.chars() {
            let Some(v) = mp.get(&c) else {
                return String::new();
            };
            res.push_str(v);
        }
        res
    }

    fn encrypt(&self, word1: String) -> String {
        Self::encrypt_word(&self.mp, &word1)
    }

    fn decrypt(&self, word2: String) -> i32 {
        *self.cnt.get(&word2).unwrap_or(&0)
    }
}

fn main() {
    let enc = Encrypter::new(
        vec!['a', 'b', 'c', 'd'],
        vec!["ei".into(), "zf".into(), "ei".into(), "am".into()],
        vec!["abcd".into(), "acbd".into(), "adbc".into(), "badc".into(), "dacb".into(), "cadb".into(), "cbda".into(), "abad".into()],
    );
    println!("{}", enc.encrypt("abcd".into()));
}

#[cfg(test)]
mod tests {
    use super::Encrypter;

    #[test]
    fn example() {
        let enc = Encrypter::new(
            vec!['a', 'b', 'c', 'd'],
            vec!["ei".into(), "zf".into(), "ei".into(), "am".into()],
            vec![
                "abcd".into(),
                "acbd".into(),
                "adbc".into(),
                "badc".into(),
                "dacb".into(),
                "cadb".into(),
                "cbda".into(),
                "abad".into(),
            ],
        );
        assert_eq!(enc.encrypt("abcd".into()), "eizfeiam");
        assert_eq!(enc.decrypt("eizfeiam".into()), 2);
        assert_eq!(enc.decrypt("eizfei".into()), 0);
    }
}
