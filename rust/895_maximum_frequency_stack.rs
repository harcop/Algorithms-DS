/// LeetCode #895 - Maximum Frequency Stack
use std::collections::HashMap;

struct FreqStack {
    freq: HashMap<i32, i32>,
    groups: HashMap<i32, Vec<i32>>,
    maxf: i32,
}

impl FreqStack {
    fn new() -> Self {
        FreqStack {
            freq: HashMap::new(),
            groups: HashMap::new(),
            maxf: 0,
        }
    }

    fn push(&mut self, val: i32) {
        let f = self.freq.entry(val).or_insert(0);
        *f += 1;
        let nf = *f;
        self.maxf = self.maxf.max(nf);
        self.groups.entry(nf).or_insert_with(Vec::new).push(val);
    }

    fn pop(&mut self) -> i32 {
        let g = self.groups.get_mut(&self.maxf).unwrap();
        let val = g.pop().unwrap();
        if g.is_empty() {
            self.maxf -= 1;
        }
        let e = self.freq.get_mut(&val).unwrap();
        *e -= 1;
        val
    }
}

fn main() {
    let mut fs = FreqStack::new();
    fs.push(5);
    fs.push(7);
    fs.push(5);
    fs.push(7);
    fs.push(4);
    fs.push(5);
    println!("{}", fs.pop());
}

#[cfg(test)]
mod tests {
    use super::FreqStack;

    #[test]
    fn example_one() {
        let mut fs = FreqStack::new();
        fs.push(5);
        fs.push(7);
        fs.push(5);
        fs.push(7);
        fs.push(4);
        fs.push(5);
        assert_eq!(fs.pop(), 5);
        assert_eq!(fs.pop(), 7);
        assert_eq!(fs.pop(), 5);
        assert_eq!(fs.pop(), 4);
    }
}
