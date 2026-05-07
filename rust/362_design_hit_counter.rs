/// LeetCode #362 - Design Hit Counter (queue of timestamps, prune < now-300)
use std::collections::VecDeque;

struct HitCounter {
    q: VecDeque<i32>,
}

impl HitCounter {
    fn new() -> Self {
        HitCounter { q: VecDeque::new() }
    }

    fn hit(&mut self, timestamp: i32) {
        self.q.push_back(timestamp);
        self.prune(timestamp);
    }

    fn get_hits(&mut self, timestamp: i32) -> i32 {
        self.prune(timestamp);
        self.q.len() as i32
    }

    fn prune(&mut self, t: i32) {
        let lo = t - 300;
        while let Some(&f) = self.q.front() {
            if f <= lo {
                self.q.pop_front();
            } else {
                break;
            }
        }
    }
}

fn main() {
    let mut h = HitCounter::new();
    h.hit(1);
    println!("{}", h.get_hits(300));
}

#[cfg(test)]
mod tests {
    use super::HitCounter;

    #[test]
    fn ex() {
        let mut c = HitCounter::new();
        c.hit(1);
        c.hit(2);
        c.hit(3);
        assert_eq!(c.get_hits(4), 3);
        c.hit(300);
        assert_eq!(c.get_hits(300), 4);
        assert_eq!(c.get_hits(301), 3);
    }
}
