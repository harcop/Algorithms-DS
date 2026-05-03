/// LeetCode #225 - Implement Stack using Queues
use std::collections::VecDeque;

pub struct MyStack {
    q: VecDeque<i32>,
}

impl MyStack {
    fn new() -> Self {
        MyStack {
            q: VecDeque::new(),
        }
    }

    fn push(&mut self, x: i32) {
        let n = self.q.len();
        self.q.push_back(x);
        for _ in 0..n {
            let v = self.q.pop_front().unwrap();
            self.q.push_back(v);
        }
    }

    fn pop(&mut self) -> i32 {
        self.q.pop_front().unwrap()
    }

    fn top(&self) -> i32 {
        *self.q.front().unwrap()
    }

    fn empty(&self) -> bool {
        self.q.is_empty()
    }
}

fn main() {
    let mut s = MyStack::new();
    s.push(1);
    println!("{}", s.top());
}

#[cfg(test)]
mod tests {
    use super::MyStack;

    #[test]
    fn example() {
        let mut s = MyStack::new();
        s.push(1);
        s.push(2);
        assert_eq!(s.top(), 2);
        assert_eq!(s.pop(), 2);
        assert!(!s.empty());
    }
}
