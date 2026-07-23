/// LeetCode #2627 - Debounce (JS problem; simulated timer analogue)
struct Debouncer {
    delay: i32,
    pending: Option<(i32, Vec<i32>)>, // (fire_at, args)
}

impl Debouncer {
    fn new(delay: i32) -> Self {
        Debouncer {
            delay,
            pending: None,
        }
    }

    /// Schedule a call at absolute time `t` with `inputs`.
    fn call(&mut self, t: i32, inputs: Vec<i32>) {
        self.pending = Some((t + self.delay, inputs));
    }

    /// Advance to time `now` and return any fired call (time, inputs).
    fn advance(&mut self, now: i32) -> Option<(i32, Vec<i32>)> {
        if let Some((fire_at, _)) = &self.pending {
            if *fire_at <= now {
                return self.pending.take();
            }
        }
        None
    }
}

fn main() {
    let mut d = Debouncer::new(50);
    d.call(50, vec![1]);
    d.call(75, vec![2]);
    println!("{:?}", d.advance(125));
}

#[cfg(test)]
mod tests {
    use super::Debouncer;

    #[test]
    fn example_one() {
        let mut d = Debouncer::new(50);
        d.call(50, vec![1]);
        assert!(d.advance(74).is_none());
        d.call(75, vec![2]);
        assert_eq!(d.advance(125), Some((125, vec![2])));
    }

    #[test]
    fn example_two() {
        let mut d = Debouncer::new(20);
        d.call(50, vec![1]);
        assert_eq!(d.advance(70), Some((70, vec![1])));
        d.call(100, vec![2]);
        assert_eq!(d.advance(120), Some((120, vec![2])));
    }

    #[test]
    fn example_three() {
        let mut d = Debouncer::new(150);
        d.call(50, vec![1, 2]);
        assert_eq!(d.advance(200), Some((200, vec![1, 2])));
        d.call(300, vec![3, 4]);
        d.call(300, vec![5, 6]);
        assert_eq!(d.advance(450), Some((450, vec![5, 6])));
    }
}
