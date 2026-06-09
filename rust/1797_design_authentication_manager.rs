/// LeetCode #1797 - Design Authentication Manager
use std::collections::HashMap;

pub struct AuthenticationManager {
    ttl: i32,
    tokens: HashMap<String, i32>,
}

impl AuthenticationManager {
    fn new(time_to_live: i32) -> Self {
        AuthenticationManager {
            ttl: time_to_live,
            tokens: HashMap::new(),
        }
    }

    fn generate(&mut self, token_id: String, current_time: i32) {
        self.tokens.insert(token_id, current_time + self.ttl);
    }

    fn renew(&mut self, token_id: String, current_time: i32) {
        if self
            .tokens
            .get(&token_id)
            .copied()
            .map_or(true, |exp| exp <= current_time)
        {
            return;
        }
        self.tokens.insert(token_id, current_time + self.ttl);
    }

    fn count_unexpired_tokens(&self, current_time: i32) -> i32 {
        self.tokens
            .values()
            .filter(|&&exp| exp > current_time)
            .count() as i32
    }
}

fn main() {
    let mut mgr = AuthenticationManager::new(5);
    mgr.renew("aaa".into(), 1);
    mgr.generate("aaa".into(), 2);
    println!("{}", mgr.count_unexpired_tokens(6));
}

#[cfg(test)]
mod tests {
    use super::AuthenticationManager;

    #[test]
    fn example_one() {
        let mut mgr = AuthenticationManager::new(5);
        mgr.renew("aaa".into(), 1);
        mgr.generate("aaa".into(), 2);
        assert_eq!(mgr.count_unexpired_tokens(6), 1);
        mgr.generate("bbb".into(), 7);
        mgr.renew("aaa".into(), 8);
        mgr.renew("bbb".into(), 10);
        assert_eq!(mgr.count_unexpired_tokens(15), 0);
    }
}
