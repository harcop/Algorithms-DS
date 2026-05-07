/// LeetCode #346 - Moving Average from Data Stream
use std::collections::VecDeque;

struct MovingAverage {
    buf: VecDeque<i32>,
    cap: usize,
    sum: i64,
}

impl MovingAverage {
    fn new(size: i32) -> Self {
        MovingAverage {
            buf: VecDeque::new(),
            cap: size as usize,
            sum: 0,
        }
    }

    fn next(&mut self, val: i32) -> f64 {
        self.buf.push_back(val);
        self.sum += val as i64;
        if self.buf.len() > self.cap {
            let v = self.buf.pop_front().unwrap();
            self.sum -= v as i64;
        }
        self.sum as f64 / self.buf.len() as f64
    }
}

fn main() {
    let mut m = MovingAverage::new(3);
    println!("{}", m.next(1));
    println!("{}", m.next(10));
    println!("{}", m.next(3));
    println!("{}", m.next(5));
}

#[cfg(test)]
mod tests {
    use super::MovingAverage;

    #[test]
    fn rolling() {
        let mut m = MovingAverage::new(3);
        assert!((m.next(1) - 1.0).abs() < 1e-9);
        assert!((m.next(10) - 5.5).abs() < 1e-9);
        assert!((m.next(3) - 4.666666666666667).abs() < 1e-6);
        assert!((m.next(5) - 6.0).abs() < 1e-9);
    }
}
