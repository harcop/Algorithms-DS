/// LeetCode #707 - Design Linked List
struct Node {
    val: i32,
    next: Option<Box<Node>>,
}

struct MyLinkedList {
    head: Option<Box<Node>>,
    size: i32,
}

impl MyLinkedList {
    fn new() -> Self {
        Self { head: None, size: 0 }
    }

    fn get(&self, index: i32) -> i32 {
        if index < 0 || index >= self.size {
            return -1;
        }
        let mut cur = self.head.as_ref();
        for _ in 0..index {
            cur = cur.unwrap().next.as_ref();
        }
        cur.unwrap().val
    }

    fn add_at_head(&mut self, val: i32) {
        let node = Box::new(Node {
            val,
            next: self.head.take(),
        });
        self.head = Some(node);
        self.size += 1;
    }

    fn add_at_tail(&mut self, val: i32) {
        let node = Box::new(Node { val, next: None });
        if self.head.is_none() {
            self.head = Some(node);
        } else {
            let mut cur = self.head.as_mut().unwrap();
            while cur.next.is_some() {
                cur = cur.next.as_mut().unwrap();
            }
            cur.next = Some(node);
        }
        self.size += 1;
    }

    fn add_at_index(&mut self, index: i32, val: i32) {
        if index > self.size {
            return;
        }
        if index <= 0 {
            self.add_at_head(val);
            return;
        }
        let mut cur = self.head.as_mut().unwrap();
        for _ in 0..index - 1 {
            cur = cur.next.as_mut().unwrap();
        }
        let node = Box::new(Node {
            val,
            next: cur.next.take(),
        });
        cur.next = Some(node);
        self.size += 1;
    }

    fn delete_at_index(&mut self, index: i32) {
        if index < 0 || index >= self.size {
            return;
        }
        if index == 0 {
            self.head = self.head.take().unwrap().next;
        } else {
            let mut cur = self.head.as_mut().unwrap();
            for _ in 0..index - 1 {
                cur = cur.next.as_mut().unwrap();
            }
            let next_next = cur.next.as_mut().unwrap().next.take();
            cur.next = next_next;
        }
        self.size -= 1;
    }
}

fn main() {
    let mut l = MyLinkedList::new();
    l.add_at_head(1);
    println!("{}", l.get(0));
}

#[cfg(test)]
mod tests {
    use super::MyLinkedList;

    #[test]
    fn example() {
        let mut l = MyLinkedList::new();
        l.add_at_head(1);
        l.add_at_tail(3);
        l.add_at_index(1, 2);
        assert_eq!(l.get(1), 2);
        l.delete_at_index(1);
        assert_eq!(l.get(1), 3);
        assert_eq!(l.get(2), -1);
    }
}
