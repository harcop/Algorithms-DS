/// LeetCode #1188 - Design Bounded Blocking Queue
use std::collections::VecDeque;

struct BoundedBlockingQueue {
    capacity: usize,
    q: VecDeque<i32>,
}

impl BoundedBlockingQueue {
    fn new(capacity: i32) -> Self {
        BoundedBlockingQueue {
            capacity: capacity as usize,
            q: VecDeque::new(),
        }
    }

    fn enqueue(&mut self, element: i32) {
        if self.q.len() < self.capacity {
            self.q.push_back(element);
        }
    }

    fn dequeue(&mut self) -> i32 {
        self.q.pop_front().unwrap_or(-1)
    }

    fn size(&self) -> i32 {
        self.q.len() as i32
    }
}

fn main() {
    let mut q = BoundedBlockingQueue::new(2);
    q.enqueue(1);
    println!("{}", q.dequeue());
}

#[cfg(test)]
mod tests {
    use super::BoundedBlockingQueue;

    #[test]
    fn example() {
        let mut q = BoundedBlockingQueue::new(2);
        q.enqueue(1);
        assert_eq!(q.dequeue(), 1);
        q.enqueue(0);
        q.enqueue(2);
        assert_eq!(q.size(), 2);
        assert_eq!(q.dequeue(), 0);
        assert_eq!(q.dequeue(), 2);
        assert_eq!(q.size(), 0);
    }
}
