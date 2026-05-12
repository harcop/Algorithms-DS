/// LeetCode #715 - Range Module
use std::collections::BTreeMap;

struct RangeModule {
    intervals: BTreeMap<i32, i32>,
}

impl RangeModule {
    fn new() -> Self {
        Self {
            intervals: BTreeMap::new(),
        }
    }

    fn add_range(&mut self, mut left: i32, mut right: i32) {
        let to_remove: Vec<i32> = self
            .intervals
            .range(..=right)
            .filter(|(_, &e)| e >= left)
            .map(|(&s, _)| s)
            .collect();
        for k in to_remove {
            let e = self.intervals.remove(&k).unwrap();
            left = left.min(k);
            right = right.max(e);
        }
        self.intervals.insert(left, right);
    }

    fn query_range(&self, left: i32, right: i32) -> bool {
        if let Some((_, &e)) = self.intervals.range(..=left).next_back() {
            return e >= right;
        }
        false
    }

    fn remove_range(&mut self, left: i32, right: i32) {
        let to_process: Vec<(i32, i32)> = self
            .intervals
            .range(..right)
            .filter(|(_, &e)| e > left)
            .map(|(&s, &e)| (s, e))
            .collect();
        for (s, e) in to_process {
            self.intervals.remove(&s);
            if s < left {
                self.intervals.insert(s, left);
            }
            if e > right {
                self.intervals.insert(right, e);
            }
        }
    }
}

fn main() {
    let mut r = RangeModule::new();
    r.add_range(10, 20);
    println!("{}", r.query_range(10, 14));
}

#[cfg(test)]
mod tests {
    use super::RangeModule;

    #[test]
    fn example() {
        let mut r = RangeModule::new();
        r.add_range(10, 20);
        r.remove_range(14, 16);
        assert!(r.query_range(10, 14));
        assert!(!r.query_range(13, 15));
        assert!(r.query_range(16, 17));
    }
}
