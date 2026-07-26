/// LeetCode #2665 - Counter II (JS problem; Rust struct analogue)
struct Counter {
    init: i32,
    val: i32,
}

impl Counter {
    fn new(init: i32) -> Self {
        Counter { init, val: init }
    }

    fn increment(&mut self) -> i32 {
        self.val += 1;
        self.val
    }

    fn decrement(&mut self) -> i32 {
        self.val -= 1;
        self.val
    }

    fn reset(&mut self) -> i32 {
        self.val = self.init;
        self.val
    }
}

fn main() {
    let mut c = Counter::new(5);
    println!("{} {} {}", c.increment(), c.reset(), c.decrement());
}

#[cfg(test)]
mod tests {
    use super::Counter;

    #[test]
    fn example_one() {
        let mut c = Counter::new(5);
        assert_eq!(c.increment(), 6);
        assert_eq!(c.reset(), 5);
        assert_eq!(c.decrement(), 4);
    }

    #[test]
    fn example_two() {
        let mut c = Counter::new(0);
        assert_eq!(c.increment(), 1);
        assert_eq!(c.increment(), 2);
        assert_eq!(c.decrement(), 1);
        assert_eq!(c.reset(), 0);
        assert_eq!(c.reset(), 0);
    }
}
