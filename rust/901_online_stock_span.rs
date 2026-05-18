/// LeetCode #901 - Online Stock Span
struct StockSpanner {
    stack: Vec<(i32, i32)>,
}

impl StockSpanner {
    fn new() -> Self {
        StockSpanner { stack: Vec::new() }
    }

    fn next(&mut self, price: i32) -> i32 {
        let mut span = 1;
        while let Some(&(p, s)) = self.stack.last() {
            if p <= price {
                self.stack.pop();
                span += s;
            } else {
                break;
            }
        }
        self.stack.push((price, span));
        span
    }
}

fn main() {
    let mut s = StockSpanner::new();
    println!("{}", s.next(100));
}

#[cfg(test)]
mod tests {
    use super::StockSpanner;

    #[test]
    fn example_one() {
        let mut s = StockSpanner::new();
        assert_eq!(s.next(100), 1);
        assert_eq!(s.next(80), 1);
        assert_eq!(s.next(60), 1);
        assert_eq!(s.next(70), 2);
        assert_eq!(s.next(60), 1);
        assert_eq!(s.next(75), 4);
        assert_eq!(s.next(85), 6);
    }
}
