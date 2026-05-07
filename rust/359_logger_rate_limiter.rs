/// LeetCode #359 - Logger Rate Limiter
use std::collections::HashMap;

struct Logger {
    last: HashMap<String, i32>,
}

impl Logger {
    fn new() -> Self {
        Logger {
            last: HashMap::new(),
        }
    }

    fn should_print_message(&mut self, timestamp: i32, message: String) -> bool {
        if let Some(&t) = self.last.get(&message) {
            if timestamp - t < 10 {
                return false;
            }
        }
        self.last.insert(message, timestamp);
        true
    }
}

fn main() {
    let mut log = Logger::new();
    println!("{}", log.should_print_message(1, "foo".into()));
}

#[cfg(test)]
mod tests {
    use super::Logger;

    #[test]
    fn example() {
        let mut l = Logger::new();
        assert!(l.should_print_message(1, "foo".into()));
        assert!(l.should_print_message(2, "bar".into()));
        assert!(!l.should_print_message(3, "foo".into()));
        assert!(!l.should_print_message(8, "bar".into()));
        assert!(!l.should_print_message(10, "foo".into()));
        assert!(l.should_print_message(11, "foo".into()));
    }
}
