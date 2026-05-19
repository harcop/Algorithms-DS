/// LeetCode #1116 - Print Zero Even Odd
use std::sync::atomic::{AtomicUsize, Ordering};

pub struct ZeroEvenOdd {
    n: usize,
    turn: AtomicUsize,
}

impl ZeroEvenOdd {
    pub fn new(n: usize) -> Self {
        Self {
            n,
            turn: AtomicUsize::new(0),
        }
    }

    pub fn zero(&self, print_number: impl Fn(usize)) {
        for _ in 0..self.n {
            while self.turn.load(Ordering::SeqCst) != 0 {
                std::hint::spin_loop();
            }
            print_number(0);
            self.turn.store(1, Ordering::SeqCst);
        }
    }

    pub fn even(&self, print_number: impl Fn(usize)) {
        for i in (2..=self.n).step_by(2) {
            while self.turn.load(Ordering::SeqCst) != 2 {
                std::hint::spin_loop();
            }
            print_number(i);
            self.turn.store(0, Ordering::SeqCst);
        }
    }

    pub fn odd(&self, print_number: impl Fn(usize)) {
        for i in (1..self.n).step_by(2) {
            while self.turn.load(Ordering::SeqCst) != 1 {
                std::hint::spin_loop();
            }
            print_number(i);
            self.turn.store(2, Ordering::SeqCst);
        }
    }
}

fn main() {
    let z = ZeroEvenOdd::new(2);
    z.zero(|x| print!("{x}"));
}

#[cfg(test)]
mod tests {
    use super::ZeroEvenOdd;

    #[test]
    fn smoke() {
        let z = ZeroEvenOdd::new(1);
        z.zero(|_| {});
    }
}
