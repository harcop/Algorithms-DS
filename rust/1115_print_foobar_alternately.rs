/// LeetCode #1115 - Print FooBar Alternately
use std::sync::atomic::{AtomicUsize, Ordering};

pub struct FooBar {
    n: usize,
    turn: AtomicUsize,
}

impl FooBar {
    pub fn new(n: usize) -> Self {
        Self {
            n,
            turn: AtomicUsize::new(0),
        }
    }

    pub fn foo(&self, print_foo: impl Fn()) {
        for _ in 0..self.n {
            while self.turn.load(Ordering::SeqCst) != 0 {
                std::hint::spin_loop();
            }
            print_foo();
            self.turn.store(1, Ordering::SeqCst);
        }
    }

    pub fn bar(&self, print_bar: impl Fn()) {
        for _ in 0..self.n {
            while self.turn.load(Ordering::SeqCst) != 1 {
                std::hint::spin_loop();
            }
            print_bar();
            self.turn.store(0, Ordering::SeqCst);
        }
    }
}

fn main() {
    let fb = FooBar::new(2);
    fb.foo(|| print!("foo"));
    fb.bar(|| print!("bar"));
}

#[cfg(test)]
mod tests {
    use super::FooBar;
    use std::sync::Mutex;

    #[test]
    fn smoke() {
        let fb = FooBar::new(1);
        fb.foo(|| {});
        fb.bar(|| {});
    }
}
