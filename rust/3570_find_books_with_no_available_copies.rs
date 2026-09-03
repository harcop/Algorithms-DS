/// LeetCode #3570 - Find Books with No Available Copies (SQL; Rust analogue)
use std::collections::HashMap;

fn find_books_with_no_available_copies(
    books: Vec<(i32, String, String, String, i32, i32)>,
    records: Vec<(i32, i32, String, String, Option<String>)>,
) -> Vec<(i32, String, String, String, i32, i32)> {
    let mut current: HashMap<i32, i32> = HashMap::new();
    for (_rid, book_id, _, _, ret) in records {
        if ret.is_none() {
            *current.entry(book_id).or_insert(0) += 1;
        }
    }
    let mut ans = Vec::new();
    for (book_id, title, author, genre, year, total) in books {
        if let Some(&borrowers) = current.get(&book_id) {
            if borrowers == total {
                ans.push((book_id, title, author, genre, year, borrowers));
            }
        }
    }
    ans.sort_by(|a, b| b.5.cmp(&a.5).then(a.1.cmp(&b.1)));
    ans
}

fn main() {
    println!("{:?}", find_books_with_no_available_copies(vec![], vec![]));
}

#[cfg(test)]
mod tests {
    use super::find_books_with_no_available_copies;

    #[test]
    fn example() {
        let books = vec![
            (1, "The Great Gatsby".into(), "F. Scott".into(), "Fiction".into(), 1925, 3),
            (2, "To Kill a Mockingbird".into(), "Harper Lee".into(), "Fiction".into(), 1960, 3),
            (3, "1984".into(), "George Orwell".into(), "Dystopian".into(), 1949, 1),
            (4, "Pride and Prejudice".into(), "Jane Austen".into(), "Romance".into(), 1813, 2),
            (5, "The Catcher in the Rye".into(), "J.D. Salinger".into(), "Fiction".into(), 1951, 1),
            (6, "Brave New World".into(), "Aldous Huxley".into(), "Dystopian".into(), 1932, 4),
        ];
        let records = vec![
            (1, 1, "Alice Smith".into(), "2024-01-15".into(), None),
            (2, 1, "Bob Johnson".into(), "2024-01-20".into(), None),
            (3, 2, "Carol White".into(), "2024-01-10".into(), Some("2024-01-25".into())),
            (4, 3, "David Brown".into(), "2024-02-01".into(), None),
            (5, 4, "Emma Wilson".into(), "2024-01-05".into(), None),
            (6, 5, "Frank Davis".into(), "2024-01-18".into(), Some("2024-02-10".into())),
            (7, 1, "Grace Miller".into(), "2024-02-05".into(), None),
            (8, 6, "Henry Taylor".into(), "2024-01-12".into(), None),
            (9, 2, "Ivan Clark".into(), "2024-02-12".into(), None),
            (10, 2, "Jane Adams".into(), "2024-02-15".into(), None),
        ];
        assert_eq!(
            find_books_with_no_available_copies(books, records),
            vec![
                (1, "The Great Gatsby".into(), "F. Scott".into(), "Fiction".into(), 1925, 3),
                (3, "1984".into(), "George Orwell".into(), "Dystopian".into(), 1949, 1),
            ]
        );
    }
}
