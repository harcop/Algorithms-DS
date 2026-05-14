/// LeetCode #731 - My Calendar II
struct MyCalendarTwo {
    books: Vec<(i32, i32)>,
}

impl MyCalendarTwo {
    fn new() -> Self {
        Self { books: vec![] }
    }

    fn max_overlap_if_add(&self, start: i32, end: i32) -> i32 {
        let mut ev: Vec<(i32, i32)> = Vec::new();
        for &(s, e) in &self.books {
            ev.push((s, 1));
            ev.push((e, -1));
        }
        ev.push((start, 1));
        ev.push((end, -1));
        ev.sort_by(|a, b| a.0.cmp(&b.0).then(a.1.cmp(&b.1)));
        let mut c = 0i32;
        let mut mx = 0i32;
        for (_, d) in ev {
            c += d;
            mx = mx.max(c);
        }
        mx
    }

    fn book(&mut self, start: i32, end: i32) -> bool {
        if self.max_overlap_if_add(start, end) <= 2 {
            self.books.push((start, end));
            true
        } else {
            false
        }
    }
}

fn main() {
    let mut c = MyCalendarTwo::new();
    println!("{} {}", c.book(10, 20), c.book(50, 60));
}

#[cfg(test)]
mod tests {
    use super::MyCalendarTwo;

    #[test]
    fn example() {
        let mut c = MyCalendarTwo::new();
        assert!(c.book(10, 20));
        assert!(c.book(50, 60));
        assert!(c.book(10, 40));
        assert!(!c.book(5, 15));
        assert!(c.book(5, 10));
        assert!(c.book(25, 55));
    }
}
