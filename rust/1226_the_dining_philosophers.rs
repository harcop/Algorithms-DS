/// LeetCode #1226 - The Dining Philosophers
use std::sync::atomic::{AtomicUsize, Ordering};

pub struct DiningPhilosophers {
    turn: AtomicUsize,
}

impl DiningPhilosophers {
    pub fn new() -> Self {
        Self {
            turn: AtomicUsize::new(0),
        }
    }

    pub fn wants_to_eat(
        &self,
        philosopher: i32,
        pick_left: impl Fn(),
        pick_right: impl Fn(),
        eat: impl Fn(),
        put_left: impl Fn(),
        put_right: impl Fn(),
    ) {
        let p = philosopher as usize;
        while self.turn.load(Ordering::SeqCst) != p {
            std::hint::spin_loop();
        }
        pick_left();
        pick_right();
        eat();
        put_left();
        put_right();
        self.turn.store((p + 1) % 5, Ordering::SeqCst);
    }
}

fn main() {
    let dp = DiningPhilosophers::new();
    dp.wants_to_eat(0, || {}, || {}, || {}, || {}, || {});
}

#[cfg(test)]
mod tests {
    use super::DiningPhilosophers;

    #[test]
    fn smoke() {
        let dp = DiningPhilosophers::new();
        for p in 0..5 {
            dp.wants_to_eat(p, || {}, || {}, || {}, || {}, || {});
        }
    }
}
