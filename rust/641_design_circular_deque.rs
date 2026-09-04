/// LeetCode #641 - Design Circular Deque
struct MyCircularDeque {
    data: Vec<i32>,
    head: usize,
    size: usize,
    cap: usize,
}

impl MyCircularDeque {
    fn new(k: i32) -> Self {
        let k = k as usize;
        Self {
            data: vec![0; k],
            head: 0,
            size: 0,
            cap: k,
        }
    }

    fn insert_front(&mut self, value: i32) -> bool {
        if self.is_full() {
            return false;
        }
        self.head = (self.head + self.cap - 1) % self.cap;
        self.data[self.head] = value;
        self.size += 1;
        true
    }

    fn insert_last(&mut self, value: i32) -> bool {
        if self.is_full() {
            return false;
        }
        let tail = (self.head + self.size) % self.cap;
        self.data[tail] = value;
        self.size += 1;
        true
    }

    fn delete_front(&mut self) -> bool {
        if self.is_empty() {
            return false;
        }
        self.head = (self.head + 1) % self.cap;
        self.size -= 1;
        true
    }

    fn delete_last(&mut self) -> bool {
        if self.is_empty() {
            return false;
        }
        self.size -= 1;
        true
    }

    fn get_front(&self) -> i32 {
        if self.is_empty() {
            -1
        } else {
            self.data[self.head]
        }
    }

    fn get_rear(&self) -> i32 {
        if self.is_empty() {
            -1
        } else {
            self.data[(self.head + self.size - 1) % self.cap]
        }
    }

    fn is_empty(&self) -> bool {
        self.size == 0
    }

    fn is_full(&self) -> bool {
        self.size == self.cap
    }
}

fn main() {
    let mut q = MyCircularDeque::new(3);
    println!("{}", q.insert_last(1));
}

#[cfg(test)]
mod tests {
    use super::MyCircularDeque;

    #[test]
    fn example() {
        let mut q = MyCircularDeque::new(3);
        assert!(q.insert_last(1));
        assert!(q.insert_last(2));
        assert!(q.insert_front(3));
        assert!(!q.insert_front(4));
        assert_eq!(q.get_rear(), 2);
        assert!(q.is_full());
        assert!(q.delete_last());
        assert!(q.insert_front(4));
        assert_eq!(q.get_front(), 4);
    }
}
