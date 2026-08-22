/// LeetCode #3369 - Design an Array Statistics Tracker
use std::cmp::Reverse;
use std::collections::{BTreeMap, BTreeSet, HashMap, VecDeque};

struct StatisticsTracker {
    q: VecDeque<i32>,
    sum: i64,
    cnt: HashMap<i32, i32>,
    sorted: BTreeMap<i32, i32>,
    modes: BTreeSet<(Reverse<i32>, i32)>,
}

impl StatisticsTracker {
    fn new() -> Self {
        StatisticsTracker {
            q: VecDeque::new(),
            sum: 0,
            cnt: HashMap::new(),
            sorted: BTreeMap::new(),
            modes: BTreeSet::new(),
        }
    }

    fn add_number(&mut self, number: i32) {
        self.q.push_back(number);
        self.sum += number as i64;
        let old = *self.cnt.get(&number).unwrap_or(&0);
        if old > 0 {
            self.modes.remove(&(Reverse(old), number));
        }
        let new = old + 1;
        self.cnt.insert(number, new);
        self.modes.insert((Reverse(new), number));
        *self.sorted.entry(number).or_insert(0) += 1;
    }

    fn remove_first_added_number(&mut self) {
        let number = self.q.pop_front().unwrap();
        self.sum -= number as i64;
        let old = self.cnt[&number];
        self.modes.remove(&(Reverse(old), number));
        let new = old - 1;
        if new == 0 {
            self.cnt.remove(&number);
            self.sorted.remove(&number);
        } else {
            self.cnt.insert(number, new);
            self.sorted.insert(number, new);
            self.modes.insert((Reverse(new), number));
        }
    }

    fn get_mean(&self) -> i32 {
        (self.sum / self.q.len() as i64) as i32
    }

    fn get_median(&self) -> i32 {
        let mut rem = self.q.len() / 2;
        for (&val, &c) in &self.sorted {
            if rem < c as usize {
                return val;
            }
            rem -= c as usize;
        }
        unreachable!()
    }

    fn get_mode(&self) -> i32 {
        self.modes.iter().next().unwrap().1
    }
}

fn main() {
    let mut obj = StatisticsTracker::new();
    obj.add_number(4);
    obj.add_number(4);
    obj.add_number(2);
    obj.add_number(3);
    println!("{} {} {}", obj.get_mean(), obj.get_median(), obj.get_mode());
}

#[cfg(test)]
mod tests {
    use super::StatisticsTracker;

    #[test]
    fn example1() {
        let mut obj = StatisticsTracker::new();
        obj.add_number(4);
        obj.add_number(4);
        obj.add_number(2);
        obj.add_number(3);
        assert_eq!(obj.get_mean(), 3);
        assert_eq!(obj.get_median(), 4);
        assert_eq!(obj.get_mode(), 4);
        obj.remove_first_added_number();
        assert_eq!(obj.get_mode(), 2);
    }

    #[test]
    fn example2() {
        let mut obj = StatisticsTracker::new();
        obj.add_number(9);
        obj.add_number(5);
        assert_eq!(obj.get_mean(), 7);
        obj.remove_first_added_number();
        obj.add_number(5);
        obj.add_number(6);
        obj.remove_first_added_number();
        assert_eq!(obj.get_median(), 6);
        obj.add_number(8);
        assert_eq!(obj.get_mode(), 5);
    }
}
