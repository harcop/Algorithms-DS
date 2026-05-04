/// LeetCode #295 - Find Median from Data Stream
use std::cmp::Reverse;
use std::collections::BinaryHeap;

pub struct MedianFinder {
    lo: BinaryHeap<i32>,
    hi: BinaryHeap<Reverse<i32>>,
}

impl MedianFinder {
    fn new() -> Self {
        MedianFinder {
            lo: BinaryHeap::new(),
            hi: BinaryHeap::new(),
        }
    }

    fn add_num(&mut self, num: i32) {
        self.hi.push(Reverse(num));
        if let Some(Reverse(v)) = self.hi.pop() {
            self.lo.push(v);
        }
        if self.lo.len() > self.hi.len() + 1 {
            if let Some(v) = self.lo.pop() {
                self.hi.push(Reverse(v));
            }
        }
    }

    fn find_median(&self) -> f64 {
        if self.lo.len() > self.hi.len() {
            *self.lo.peek().unwrap() as f64
        } else {
            (*self.lo.peek().unwrap() + self.hi.peek().unwrap().0) as f64 / 2.0
        }
    }
}

fn main() {
    let mut m = MedianFinder::new();
    m.add_num(1);
    m.add_num(2);
    println!("{}", m.find_median());
}

#[cfg(test)]
mod tests {
    use super::MedianFinder;

    #[test]
    fn example() {
        let mut m = MedianFinder::new();
        m.add_num(1);
        m.add_num(2);
        assert_eq!(m.find_median(), 1.5);
        m.add_num(3);
        assert_eq!(m.find_median(), 2.0);
    }
}
