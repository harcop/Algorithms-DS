/// LeetCode #1845 - Seat Reservation Manager
use std::cmp::Reverse;
use std::collections::BinaryHeap;

pub struct SeatManager {
    heap: BinaryHeap<Reverse<i32>>,
}

impl SeatManager {
    fn new(n: i32) -> Self {
        SeatManager {
            heap: (1..=n).map(Reverse).collect(),
        }
    }

    fn reserve(&mut self) -> i32 {
        self.heap.pop().unwrap().0
    }

    fn unreserve(&mut self, seat_number: i32) {
        self.heap.push(Reverse(seat_number));
    }
}

fn main() {
    let mut mgr = SeatManager::new(5);
    println!("{}", mgr.reserve());
}

#[cfg(test)]
mod tests {
    use super::SeatManager;

    #[test]
    fn example_one() {
        let mut mgr = SeatManager::new(5);
        assert_eq!(mgr.reserve(), 1);
        assert_eq!(mgr.reserve(), 2);
        mgr.unreserve(2);
        assert_eq!(mgr.reserve(), 2);
        assert_eq!(mgr.reserve(), 3);
        assert_eq!(mgr.reserve(), 4);
        assert_eq!(mgr.reserve(), 5);
        mgr.unreserve(5);
    }
}
