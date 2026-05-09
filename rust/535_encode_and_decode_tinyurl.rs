/// LeetCode #535 - Encode and Decode TinyURL
use std::collections::HashMap;

struct Codec {
    url_to_key: HashMap<String, String>,
    key_to_url: HashMap<String, String>,
    next: i32,
}

impl Codec {
    fn new() -> Self {
        Self {
            url_to_key: HashMap::new(),
            key_to_url: HashMap::new(),
            next: 0,
        }
    }

    fn encode(&mut self, long_url: String) -> String {
        if let Some(k) = self.url_to_key.get(&long_url) {
            return format!("http://tinyurl.com/{}", k);
        }
        let k = self.next.to_string();
        self.next += 1;
        self.url_to_key.insert(long_url.clone(), k.clone());
        self.key_to_url.insert(k.clone(), long_url);
        format!("http://tinyurl.com/{}", k)
    }

    fn decode(&self, short_url: String) -> String {
        let k = short_url.trim_start_matches("http://tinyurl.com/");
        self.key_to_url.get(k).cloned().unwrap_or_default()
    }
}

fn main() {
    let mut c = Codec::new();
    let u = "https://leetcode.com/problems/design-tinyurl".to_string();
    let s = c.encode(u.clone());
    println!("{}", c.decode(s));
}

#[cfg(test)]
mod tests {
    use super::Codec;

    #[test]
    fn round_trip() {
        let mut c = Codec::new();
        let u = "https://example.com/a".to_string();
        let s = c.encode(u.clone());
        assert_eq!(c.decode(s), u);
    }
}
