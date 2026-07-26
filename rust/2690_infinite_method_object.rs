/// LeetCode #2690 - Infinite Method Object (JS problem; Rust HashMap analogue)
/// Any "method" key returns a closure that yields that key's name.
use std::collections::HashMap;

fn create_infinite_object() -> HashMap<String, Box<dyn Fn() -> String>> {
    // Proxy analogue: callers use `call_method` instead of dynamic property access.
    HashMap::new()
}

fn call_method(_obj: &HashMap<String, Box<dyn Fn() -> String>>, method: &str) -> String {
    method.to_string()
}

fn main() {
    let obj = create_infinite_object();
    println!("{}", call_method(&obj, "abc123"));
}

#[cfg(test)]
mod tests {
    use super::{call_method, create_infinite_object};

    #[test]
    fn example_one() {
        let obj = create_infinite_object();
        assert_eq!(call_method(&obj, "abc123"), "abc123");
    }

    #[test]
    fn example_two() {
        let obj = create_infinite_object();
        assert_eq!(call_method(&obj, ".-qw73n|^2It"), ".-qw73n|^2It");
    }
}
