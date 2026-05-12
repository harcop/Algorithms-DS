/// LeetCode #716 - Max Stack
struct MaxStack {
    data: Vec<i32>,
}

impl MaxStack {
    fn new() -> Self {
        Self { data: vec![] }
    }

    fn push(&mut self, x: i32) {
        self.data.push(x);
    }

    fn pop(&mut self) -> i32 {
        self.data.pop().unwrap()
    }

    fn top(&self) -> i32 {
        *self.data.last().unwrap()
    }

    fn peek_max(&self) -> i32 {
        *self.data.iter().max().unwrap()
    }

    fn pop_max(&mut self) -> i32 {
        let m = self.peek_max();
        for i in (0..self.data.len()).rev() {
            if self.data[i] == m {
                self.data.remove(i);
                return m;
            }
        }
        unreachable!()
    }
}

fn main() {
    let mut s = MaxStack::new();
    s.push(5);
    s.push(1);
    s.push(5);
    println!("{}", s.peek_max());
}

#[cfg(test)]
mod tests {
    use super::MaxStack;

    #[test]
    fn example() {
        let mut s = MaxStack::new();
        s.push(5);
        s.push(1);
        s.push(5);
        assert_eq!(s.top(), 5);
        assert_eq!(s.pop_max(), 5);
        assert_eq!(s.top(), 1);
        assert_eq!(s.peek_max(), 5);
        assert_eq!(s.pop(), 1);
        assert_eq!(s.top(), 5);
    }
}
