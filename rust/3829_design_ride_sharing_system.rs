/// LeetCode #3829 - Design Ride Sharing System
use std::collections::{HashSet, VecDeque};

struct RideSharingSystem {
    riders: VecDeque<i32>,
    drivers: VecDeque<i32>,
    waiting: HashSet<i32>,
}

impl RideSharingSystem {
    fn new() -> Self {
        Self {
            riders: VecDeque::new(),
            drivers: VecDeque::new(),
            waiting: HashSet::new(),
        }
    }

    fn add_rider(&mut self, rider_id: i32) {
        self.riders.push_back(rider_id);
        self.waiting.insert(rider_id);
    }

    fn add_driver(&mut self, driver_id: i32) {
        self.drivers.push_back(driver_id);
    }

    fn match_driver_with_rider(&mut self) -> Vec<i32> {
        while let Some(&r) = self.riders.front() {
            if self.waiting.contains(&r) {
                break;
            }
            self.riders.pop_front();
        }
        if self.riders.is_empty() || self.drivers.is_empty() {
            return vec![-1, -1];
        }
        let rider = self.riders.pop_front().unwrap();
        self.waiting.remove(&rider);
        let driver = self.drivers.pop_front().unwrap();
        vec![driver, rider]
    }

    fn cancel_rider(&mut self, rider_id: i32) {
        self.waiting.remove(&rider_id);
    }
}

fn main() {
    let mut s = RideSharingSystem::new();
    s.add_rider(3);
    s.add_driver(2);
    println!("{:?}", s.match_driver_with_rider());
}

#[cfg(test)]
mod tests {
    use super::RideSharingSystem;

    #[test]
    fn example1() {
        let mut s = RideSharingSystem::new();
        s.add_rider(3);
        s.add_driver(2);
        s.add_rider(1);
        assert_eq!(s.match_driver_with_rider(), vec![2, 3]);
        s.add_driver(5);
        s.cancel_rider(3);
        assert_eq!(s.match_driver_with_rider(), vec![5, 1]);
        assert_eq!(s.match_driver_with_rider(), vec![-1, -1]);
    }

    #[test]
    fn example2() {
        let mut s = RideSharingSystem::new();
        s.add_rider(8);
        s.add_driver(8);
        s.add_driver(6);
        assert_eq!(s.match_driver_with_rider(), vec![8, 8]);
        s.add_rider(2);
        s.cancel_rider(2);
        assert_eq!(s.match_driver_with_rider(), vec![-1, -1]);
    }
}
