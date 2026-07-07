/// LeetCode #2276 - Count Integers in Intervals
use std::collections::BTreeMap;

struct CountIntervals {
    intervals: BTreeMap<i32, i32>,
    cnt: i32,
}

impl CountIntervals {
    fn new() -> Self {
        CountIntervals {
            intervals: BTreeMap::new(),
            cnt: 0,
        }
    }

    fn add(&mut self, mut left: i32, mut right: i32) {
        while self.is_overlapped(left, right) {
            let (&l, &r) = self
                .intervals
                .range(..=right)
                .next_back()
                .unwrap();
            left = left.min(l);
            right = right.max(r);
            self.intervals.remove(&l);
            self.cnt -= r - l + 1;
        }
        self.intervals.insert(left, right);
        self.cnt += right - left + 1;
    }

    fn count(&self) -> i32 {
        self.cnt
    }

    fn is_overlapped(&self, left: i32, right: i32) -> bool {
        let Some((&_, &r)) = self.intervals.range(..=right).next_back() else {
            return false;
        };
        r >= left
    }
}

fn main() {
    let mut ci = CountIntervals::new();
    ci.add(2, 3);
    ci.add(7, 10);
    println!("{}", ci.count());
}

#[cfg(test)]
mod tests {
    use super::CountIntervals;

    #[test]
    fn example_one() {
        let mut ci = CountIntervals::new();
        ci.add(2, 3);
        ci.add(7, 10);
        assert_eq!(ci.count(), 6);
        ci.add(5, 8);
        assert_eq!(ci.count(), 8);
    }
}
