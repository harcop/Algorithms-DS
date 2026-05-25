/// LeetCode #1381 - Design A Stack With Increment Operation

struct CustomStack {
    data: Vec<i32>,
    inc: Vec<i32>,
    cap: usize,
}

impl CustomStack {
    fn new(max_size: i32) -> Self {
        let cap = max_size as usize;
        Self { data: Vec::new(), inc: Vec::new(), cap }
    }

    fn push(&mut self, x: i32) {
        if self.data.len() < self.cap {
            self.data.push(x);
            self.inc.push(0);
        }
    }

    fn pop(&mut self) -> i32 {
        let v = *self.data.last().unwrap() + *self.inc.last().unwrap();
        self.data.pop();
        self.inc.pop();
        v
    }

    fn increment(&mut self, k: i32, val: i32) {
        let k = (k as usize).min(self.data.len());
        for i in 0..k {
            self.inc[i] += val;
        }
    }
}

fn main() {
    let mut s = CustomStack::new(3);
    s.push(1);
    s.push(2);
    println!("{}", s.pop());
}

#[cfg(test)]
mod tests {
    use super::CustomStack;

    #[test]
    fn example_one() {
        let mut s = CustomStack::new(3);
        s.push(1);
        s.push(2);
        assert_eq!(s.pop(), 2);
        s.push(2);
        s.push(3);
        s.push(4);
        s.increment(5, 100);
        s.increment(2, 100);
        assert_eq!(s.pop(), 103);
        assert_eq!(s.pop(), 202);
        assert_eq!(s.pop(), 201);
    }
}
