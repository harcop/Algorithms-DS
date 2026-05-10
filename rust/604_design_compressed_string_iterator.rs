/// LeetCode #604 - Design Compressed String Iterator
struct StringIterator {
    s: Vec<u8>,
    i: usize,
    cur: u8,
    cnt: usize,
}

impl StringIterator {
    fn new(compressed_string: String) -> Self {
        Self {
            s: compressed_string.into_bytes(),
            i: 0,
            cur: 0,
            cnt: 0,
        }
    }

    fn refill(&mut self) {
        self.cnt = 0;
        if self.i >= self.s.len() {
            return;
        }
        self.cur = self.s[self.i];
        self.i += 1;
        let mut n = 0usize;
        while self.i < self.s.len() && self.s[self.i].is_ascii_digit() {
            n = n * 10 + (self.s[self.i] - b'0') as usize;
            self.i += 1;
        }
        self.cnt = n;
    }

    fn next(&mut self) -> char {
        if self.cnt == 0 && self.i < self.s.len() {
            self.refill();
        }
        if self.cnt == 0 {
            return ' ';
        }
        self.cnt -= 1;
        self.cur as char
    }

    fn has_next(&self) -> bool {
        self.cnt > 0 || self.i < self.s.len()
    }
}

fn main() {
    let mut it = StringIterator::new("L1e2t1C1o1d1e1".into());
    while it.has_next() {
        print!("{}", it.next());
    }
    println!();
}

#[cfg(test)]
mod tests {
    use super::StringIterator;

    #[test]
    fn example_one() {
        let mut it = StringIterator::new("L1e2t1C1o1d1e1".into());
        let mut out = String::new();
        while it.has_next() {
            out.push(it.next());
        }
        assert_eq!(out, "LeetCode");
    }
}
