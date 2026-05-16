/// LeetCode #855 - Exam Room
use std::collections::BTreeSet;

struct ExamRoom {
    n: i32,
    seats: BTreeSet<i32>,
}

impl ExamRoom {
    fn new(n: i32) -> Self {
        ExamRoom {
            n,
            seats: BTreeSet::new(),
        }
    }

    fn seat(&mut self) -> i32 {
        if self.seats.is_empty() {
            self.seats.insert(0);
            return 0;
        }
        let mut best = 0;
        let mut dist = self.seats.first().copied().unwrap();
        let mut prev = -1;
        for &s in &self.seats {
            if prev == -1 {
                let d = s;
                if d > dist {
                    dist = d;
                    best = 0;
                }
            } else {
                let d = (s - prev) / 2;
                if d > dist {
                    dist = d;
                    best = prev + d;
                }
            }
            prev = s;
        }
        let last = self.seats.last().copied().unwrap();
        let d = self.n - 1 - last;
        if d > dist {
            best = self.n - 1;
        }
        self.seats.insert(best);
        best
    }

    fn leave(&mut self, p: i32) {
        self.seats.remove(&p);
    }
}

fn main() {
    let mut room = ExamRoom::new(10);
    println!("{}", room.seat());
    println!("{}", room.seat());
    room.leave(4);
    println!("{}", room.seat());
}

#[cfg(test)]
mod tests {
    use super::ExamRoom;

    #[test]
    fn example_one() {
        let mut room = ExamRoom::new(10);
        assert_eq!(room.seat(), 0);
        assert_eq!(room.seat(), 9);
        assert_eq!(room.seat(), 4);
        assert_eq!(room.seat(), 2);
        room.leave(4);
        assert_eq!(room.seat(), 5);
    }
}
