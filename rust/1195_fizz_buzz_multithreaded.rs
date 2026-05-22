/// LeetCode #1195 - Fizz Buzz Multithreaded
use std::sync::atomic::{AtomicUsize, Ordering};

pub struct FizzBuzz {
    n: usize,
    cur: AtomicUsize,
    phase: AtomicUsize,
}

impl FizzBuzz {
    pub fn new(n: i32) -> Self {
        Self {
            n: n as usize,
            cur: AtomicUsize::new(1),
            phase: AtomicUsize::new(0),
        }
    }

    pub fn fizz(&self, print_fizz: impl Fn()) {
        while self.phase.load(Ordering::SeqCst) != 0 {
            std::hint::spin_loop();
        }
        let i = self.cur.load(Ordering::SeqCst);
        if i > self.n {
            return;
        }
        if i % 3 == 0 && i % 5 != 0 {
            print_fizz();
        }
        self.phase.store(1, Ordering::SeqCst);
    }

    pub fn buzz(&self, print_buzz: impl Fn()) {
        while self.phase.load(Ordering::SeqCst) != 1 {
            std::hint::spin_loop();
        }
        let i = self.cur.load(Ordering::SeqCst);
        if i > self.n {
            return;
        }
        if i % 5 == 0 && i % 3 != 0 {
            print_buzz();
        }
        self.phase.store(2, Ordering::SeqCst);
    }

    pub fn fizzbuzz(&self, print_fizzbuzz: impl Fn()) {
        while self.phase.load(Ordering::SeqCst) != 2 {
            std::hint::spin_loop();
        }
        let i = self.cur.load(Ordering::SeqCst);
        if i > self.n {
            return;
        }
        if i % 15 == 0 {
            print_fizzbuzz();
        }
        self.phase.store(3, Ordering::SeqCst);
    }

    pub fn number(&self, print_number: impl Fn(i32)) {
        while self.phase.load(Ordering::SeqCst) != 3 {
            std::hint::spin_loop();
        }
        let i = self.cur.load(Ordering::SeqCst);
        if i > self.n {
            return;
        }
        if i % 3 != 0 && i % 5 != 0 {
            print_number(i as i32);
        }
        self.cur.store(i + 1, Ordering::SeqCst);
        self.phase.store(0, Ordering::SeqCst);
    }
}

fn main() {
    let fb = FizzBuzz::new(15);
    while fb.cur.load(Ordering::SeqCst) <= fb.n {
        fb.fizz(|| print!("Fizz "));
        fb.buzz(|| print!("Buzz "));
        fb.fizzbuzz(|| print!("FizzBuzz "));
        fb.number(|x| print!("{x} "));
    }
}

#[cfg(test)]
mod tests {
    use super::FizzBuzz;
    use std::sync::atomic::Ordering;

    #[test]
    fn smoke() {
        let fb = FizzBuzz::new(1);
        while fb.cur.load(Ordering::SeqCst) <= fb.n {
            fb.fizz(|| {});
            fb.buzz(|| {});
            fb.fizzbuzz(|| {});
            fb.number(|_| {});
        }
    }
}
