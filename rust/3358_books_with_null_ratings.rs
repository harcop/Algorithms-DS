/// LeetCode #3358 - Books with NULL Ratings (SQL; Rust analogue)
/// books: (book_id, title, author, published_year, rating)
fn find_unrated_books(
    books: Vec<(i32, String, String, i32, Option<f64>)>,
) -> Vec<(i32, String, String, i32)> {
    let mut ans: Vec<_> = books
        .into_iter()
        .filter(|b| b.4.is_none())
        .map(|b| (b.0, b.1, b.2, b.3))
        .collect();
    ans.sort_by_key(|b| b.0);
    ans
}

fn main() {
    let books = vec![
        (1, "The Great Gatsby".into(), "F. Scott".into(), 1925, Some(4.5)),
        (
            2,
            "To Kill a Mockingbird".into(),
            "Harper Lee".into(),
            1960,
            None,
        ),
    ];
    println!("{:?}", find_unrated_books(books));
}

#[cfg(test)]
mod tests {
    use super::find_unrated_books;

    #[test]
    fn example() {
        let books = vec![
            (1, "The Great Gatsby".into(), "F. Scott".into(), 1925, Some(4.5)),
            (
                2,
                "To Kill a Mockingbird".into(),
                "Harper Lee".into(),
                1960,
                None,
            ),
            (
                3,
                "Pride and Prejudice".into(),
                "Jane Austen".into(),
                1813,
                Some(4.8),
            ),
            (
                4,
                "The Catcher in the Rye".into(),
                "J.D. Salinger".into(),
                1951,
                None,
            ),
            (5, "Animal Farm".into(), "George Orwell".into(), 1945, Some(4.2)),
            (
                6,
                "Lord of the Flies".into(),
                "William Golding".into(),
                1954,
                None,
            ),
        ];
        assert_eq!(
            find_unrated_books(books),
            vec![
                (
                    2,
                    "To Kill a Mockingbird".into(),
                    "Harper Lee".into(),
                    1960
                ),
                (
                    4,
                    "The Catcher in the Rye".into(),
                    "J.D. Salinger".into(),
                    1951
                ),
                (6, "Lord of the Flies".into(), "William Golding".into(), 1954),
            ]
        );
    }
}
