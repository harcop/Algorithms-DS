/// LeetCode #732 - My Calendar III
struct MyCalendarThree {
    books: Vec<(i32, i32)>,
}

impl MyCalendarThree {
    fn new() -> Self {
        Self { books: vec![] }
    }

    fn book(&mut self, start: i32, end: i32) -> i32 {
        self.books.push((start, end));
        let mut ev: Vec<(i32, i32)> = Vec::new();
        for &(s, e) in &self.books {
            ev.push((s, 1));
            ev.push((e, -1));
        }
        ev.sort_by(|a, b| a.0.cmp(&b.0).then(a.1.cmp(&b.1)));
        let mut c = 0i32;
        let mut mx = 0i32;
        for (_, d) in ev {
            c += d;
            mx = mx.max(c);
        }
        mx
    }
}

fn main() {
    let mut c = MyCalendarThree::new();
    println!("{}", c.book(10, 20));
}

#[cfg(test)]
mod tests {
    use super::MyCalendarThree;

    #[test]
    fn example_one() {
        let mut c = MyCalendarThree::new();
        assert_eq!(c.book(10, 20), 1);
        assert_eq!(c.book(50, 60), 1);
        assert_eq!(c.book(10, 40), 2);
        assert_eq!(c.book(5, 15), 3);
        assert_eq!(c.book(5, 10), 3);
        assert_eq!(c.book(25, 55), 3);
    }
}
