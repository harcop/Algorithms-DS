/// LeetCode #622 - Design Circular Queue
pub struct MyCircularQueue {
    data: Vec<i32>,
    head: usize,
    size: usize,
    cap: usize,
}

impl MyCircularQueue {
    fn new(k: i32) -> Self {
        let k = k as usize;
        Self { data: vec![0; k], head: 0, size: 0, cap: k }
    }

    fn en_queue(&mut self, value: i32) -> bool {
        if self.is_full() { return false; }
        let tail = (self.head + self.size) % self.cap;
        self.data[tail] = value;
        self.size += 1;
        true
    }

    fn de_queue(&mut self) -> bool {
        if self.is_empty() { return false; }
        self.head = (self.head + 1) % self.cap;
        self.size -= 1;
        true
    }

    fn front(&self) -> i32 {
        if self.is_empty() { -1 } else { self.data[self.head] }
    }

    fn rear(&self) -> i32 {
        if self.is_empty() { -1 } else { self.data[(self.head + self.size - 1) % self.cap] }
    }

    fn is_empty(&self) -> bool { self.size == 0 }
    fn is_full(&self) -> bool { self.size == self.cap }
}

fn main() {
    let mut q = MyCircularQueue::new(3);
    q.en_queue(1);
    println!("{}", q.front());
}

#[cfg(test)]
mod tests {
    use super::MyCircularQueue;

    #[test]
    fn example() {
        let mut q = MyCircularQueue::new(3);
        assert!(q.en_queue(1));
        assert!(q.en_queue(2));
        assert!(q.en_queue(3));
        assert!(!q.en_queue(4));
        assert_eq!(q.rear(), 3);
        assert!(q.is_full());
        assert!(q.de_queue());
        assert!(q.en_queue(4));
        assert_eq!(q.rear(), 4);
    }
}
