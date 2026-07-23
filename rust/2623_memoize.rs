/// LeetCode #2623 - Memoize (JS problem; Rust HashMap cache analogue)
use std::collections::HashMap;

struct MemoSum {
    cache: HashMap<(i32, i32), i32>,
    call_count: usize,
}

impl MemoSum {
    fn new() -> Self {
        MemoSum {
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

fn fib(n: i32) -> i32 {
    if n <= 1 {
        1
    } else {
        fib(n - 1) + fib(n - 2)
    }
}

fn factorial(n: i32) -> i32 {
    if n <= 1 {
        1
    } else {
        n * factorial(n - 1)
    }
}

fn main() {
    let mut m = MemoSum::new();
    println!("{} {}", m.call(2, 2), m.call_count);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_sum() {
        let mut m = MemoSum::new();
        assert_eq!(m.call(2, 2), 4);
        assert_eq!(m.call(2, 2), 4);
        assert_eq!(m.call_count, 1);
        assert_eq!(m.call(1, 2), 3);
        assert_eq!(m.call_count, 2);
    }

    #[test]
    fn fib_and_factorial() {
        assert_eq!(fib(5), 8);
        assert_eq!(factorial(3), 6);
    }
}
