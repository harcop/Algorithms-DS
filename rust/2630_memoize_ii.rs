/// LeetCode #2630 - Memoize II (JS problem; Rust HashMap cache by argument identity key)
use std::collections::HashMap;

/// Memoizes a binary function keyed by `(a, b)` identity (Copy args).
struct MemoizeII {
    cache: HashMap<(i32, i32), i32>,
    call_count: usize,
}

impl MemoizeII {
    fn new() -> Self {
        MemoizeII {
            cache: HashMap::new(),
            call_count: 0,
        }
    }

    fn call(&mut self, a: i32, b: i32) -> i32 {
        if let Some(&v) = self.cache.get(&(a, b)) {
            return v;
        }
        self.call_count += 1;
        let v = a + b;
        self.cache.insert((a, b), v);
        v
    }
}

fn main() {
    let mut m = MemoizeII::new();
    println!("{} {}", m.call(2, 2), m.call_count);
}

#[cfg(test)]
mod tests {
    use super::MemoizeII;

    #[test]
    fn example_one() {
        let mut m = MemoizeII::new();
        assert_eq!(m.call(2, 2), 4);
        assert_eq!(m.call_count, 1);
        assert_eq!(m.call(2, 2), 4);
        assert_eq!(m.call_count, 1);
        assert_eq!(m.call(1, 2), 3);
        assert_eq!(m.call_count, 2);
    }
}
