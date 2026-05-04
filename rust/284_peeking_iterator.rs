/// LeetCode #284 - Peeking Iterator
struct PeekingIterator<I: Iterator<Item = i32>> {
    it: I,
    buf: Option<i32>,
}

impl<I: Iterator<Item = i32>> PeekingIterator<I> {
    fn new(mut it: I) -> Self {
        let buf = it.next();
        PeekingIterator { it, buf }
    }

    fn peek(&self) -> i32 {
        self.buf.unwrap()
    }

    fn next(&mut self) -> i32 {
        let cur = self.buf.take().unwrap();
        self.buf = self.it.next();
        cur
    }

    fn has_next(&self) -> bool {
        self.buf.is_some()
    }
}

fn main() {
    let v = vec![1, 2, 3].into_iter();
    let p = PeekingIterator::new(v);
    println!("{}", p.peek());
}

#[cfg(test)]
mod tests {
    use super::PeekingIterator;

    #[test]
    fn example() {
        let v = vec![1, 2, 3].into_iter();
        let mut p = PeekingIterator::new(v);
        assert_eq!(p.peek(), 1);
        assert_eq!(p.next(), 1);
        assert_eq!(p.peek(), 2);
        assert_eq!(p.next(), 2);
        assert!(p.has_next());
        assert_eq!(p.next(), 3);
        assert!(!p.has_next());
    }
}
