/// LeetCode #2671 - Frequency Tracker
use std::collections::HashMap;

struct FrequencyTracker {
    cnt: HashMap<i32, i32>,
    freq: HashMap<i32, i32>,
}

impl FrequencyTracker {
    fn new() -> Self {
        FrequencyTracker {
            cnt: HashMap::new(),
            freq: HashMap::new(),
        }
    }

    fn add(&mut self, number: i32) {
        let cur = *self.cnt.get(&number).unwrap_or(&0);
        *self.freq.entry(cur).or_insert(0) -= 1;
        self.cnt.insert(number, cur + 1);
        *self.freq.entry(cur + 1).or_insert(0) += 1;
    }

    fn delete_one(&mut self, number: i32) {
        let cur = *self.cnt.get(&number).unwrap_or(&0);
        if cur > 0 {
            *self.freq.entry(cur).or_insert(0) -= 1;
            self.cnt.insert(number, cur - 1);
            *self.freq.entry(cur - 1).or_insert(0) += 1;
        }
    }

    fn has_frequency(&self, frequency: i32) -> bool {
        *self.freq.get(&frequency).unwrap_or(&0) > 0
    }
}

fn main() {
    let mut t = FrequencyTracker::new();
    t.add(3);
    t.add(3);
    println!("{}", t.has_frequency(2));
}

#[cfg(test)]
mod tests {
    use super::FrequencyTracker;

    #[test]
    fn example_one() {
        let mut t = FrequencyTracker::new();
        t.add(3);
        t.add(3);
        assert!(t.has_frequency(2));
    }

    #[test]
    fn example_two() {
        let mut t = FrequencyTracker::new();
        t.add(1);
        t.delete_one(1);
        assert!(!t.has_frequency(1));
    }

    #[test]
    fn example_three() {
        let mut t = FrequencyTracker::new();
        assert!(!t.has_frequency(2));
        t.add(3);
        assert!(t.has_frequency(1));
    }
}
