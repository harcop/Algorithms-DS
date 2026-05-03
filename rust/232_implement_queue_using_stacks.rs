/// LeetCode #232 - Implement Queue using Stacks
pub struct MyQueue {
    in_stk: Vec<i32>,
    out_stk: Vec<i32>,
}

impl MyQueue {
    fn new() -> Self {
        MyQueue {
            in_stk: vec![],
            out_stk: vec![],
        }
    }

    fn push(&mut self, x: i32) {
        self.in_stk.push(x);
    }

    fn pop(&mut self) -> i32 {
        self.peek();
        self.out_stk.pop().unwrap()
    }

    fn peek(&mut self) -> i32 {
        if self.out_stk.is_empty() {
            while let Some(v) = self.in_stk.pop() {
                self.out_stk.push(v);
            }
        }
        *self.out_stk.last().unwrap()
    }

    fn empty(&self) -> bool {
        self.in_stk.is_empty() && self.out_stk.is_empty()
    }
}

fn main() {
    let mut q = MyQueue::new();
    q.push(1);
    println!("{}", q.peek());
}

#[cfg(test)]
mod tests {
    use super::MyQueue;

    #[test]
    fn example() {
        let mut q = MyQueue::new();
        q.push(1);
        q.push(2);
        assert_eq!(q.peek(), 1);
        assert_eq!(q.pop(), 1);
        assert!(!q.empty());
    }
}
