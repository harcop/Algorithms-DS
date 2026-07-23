/// LeetCode #2621 - Sleep (JS problem; Rust thread::sleep analogue)
use std::thread;
use std::time::{Duration, Instant};

fn sleep(millis: u64) {
    thread::sleep(Duration::from_millis(millis));
}

fn main() {
    let t = Instant::now();
    sleep(10);
    println!("{}", t.elapsed().as_millis());
}

#[cfg(test)]
mod tests {
    use super::sleep;
    use std::time::Instant;

    #[test]
    fn sleeps_at_least_requested() {
        let t = Instant::now();
        sleep(20);
        assert!(t.elapsed().as_millis() >= 20);
    }
}
