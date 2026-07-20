/// LeetCode #2526 - Find Consecutive Integers from a Data Stream
use std::collections::VecDeque;

struct DataStream {
    value: i32,
    k: usize,
    q: VecDeque<i32>,
    count: i32,
}

impl DataStream {
    fn new(value: i32, k: i32) -> Self {
        DataStream {
            value,
            k: k as usize,
            q: VecDeque::new(),
            count: 0,
        }
    }

    fn consec(&mut self, num: i32) -> bool {
        if self.q.len() == self.k {
            if self.q.pop_front().unwrap() == self.value {
                self.count -= 1;
            }
        }
        if num == self.value {
            self.count += 1;
        }
        self.q.push_back(num);
        self.count == self.k as i32
    }
}

fn main() {
    let mut ds = DataStream::new(3, 4);
    println!("{}", ds.consec(4));
}

#[cfg(test)]
mod tests {
    use super::DataStream;

    #[test]
    fn example_sequence() {
        let mut ds = DataStream::new(3, 4);
        assert!(!ds.consec(4));
        assert!(!ds.consec(4));
        assert!(!ds.consec(3));
        assert!(!ds.consec(3));
        assert!(!ds.consec(3));
        assert!(ds.consec(3));
        assert!(!ds.consec(2));
    }
}
