/// LeetCode #933 - Number of Recent Calls
use std::collections::VecDeque;

struct RecentCounter {
    q: VecDeque<i32>,
}

impl RecentCounter {
    fn new() -> Self {
        Self {
            q: VecDeque::new(),
        }
    }

    fn ping(&mut self, t: i32) -> i32 {
        self.q.push_back(t);
        while *self.q.front().unwrap() < t - 3000 {
            self.q.pop_front();
        }
        self.q.len() as i32
    }
}

fn main() {
    let mut rc = RecentCounter::new();
    println!("{} {}", rc.ping(1), rc.ping(3001));
}

#[cfg(test)]
mod tests {
    use super::RecentCounter;

    #[test]
    fn example_one() {
        let mut rc = RecentCounter::new();
        assert_eq!(rc.ping(1), 1);
        assert_eq!(rc.ping(100), 2);
        assert_eq!(rc.ping(3001), 3);
        assert_eq!(rc.ping(3002), 3);
    }
}
