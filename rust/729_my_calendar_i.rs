/// LeetCode #729 - My Calendar I
struct MyCalendar {
    books: Vec<(i32, i32)>,
}

impl MyCalendar {
    fn new() -> Self {
        Self { books: vec![] }
    }

    fn book(&mut self, start: i32, end: i32) -> bool {
        for &(s, e) in &self.books {
            if start < e && end > s {
                return false;
            }
        }
        self.books.push((start, end));
        true
    }
}

fn main() {
    let mut c = MyCalendar::new();
    println!("{} {}", c.book(10, 20), c.book(15, 25));
}

#[cfg(test)]
mod tests {
    use super::MyCalendar;

    #[test]
    fn example() {
        let mut c = MyCalendar::new();
        assert!(c.book(10, 20));
        assert!(!c.book(15, 25));
        assert!(c.book(20, 30));
    }
}
