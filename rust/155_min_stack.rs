/// LeetCode #155 - Min Stack
pub struct MinStack {
    stack: Vec<(i32, i32)>,
}

impl MinStack {
    fn new() -> Self {
        MinStack { stack: vec![] }
    }

    fn push(&mut self, val: i32) {
        let min = if let Some(&(_, m)) = self.stack.last() {
            m.min(val)
        } else {
            val
        };
        self.stack.push((val, min));
    }

    fn pop(&mut self) {
        self.stack.pop();
    }

    fn top(&self) -> i32 {
        self.stack.last().unwrap().0
    }

    fn get_min(&self) -> i32 {
        self.stack.last().unwrap().1
    }
}

fn main() {
    let mut s = MinStack::new();
    s.push(-2);
    s.push(0);
    s.push(-3);
    println!("{}", s.get_min());
}

#[cfg(test)]
mod tests {
    use super::MinStack;

    #[test]
    fn example() {
        let mut s = MinStack::new();
        s.push(-2);
        s.push(0);
        s.push(-3);
        assert_eq!(s.get_min(), -3);
        s.pop();
        assert_eq!(s.top(), 0);
        assert_eq!(s.get_min(), -2);
    }
}
