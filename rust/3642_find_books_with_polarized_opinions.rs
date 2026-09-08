/// LeetCode #3642 - Find Books with Polarized Opinions (SQL; Rust analogue)
use std::collections::HashMap;

fn find_polarized_books(
    books: Vec<(i32, String, String, String, i32)>,
    sessions: Vec<(i32, i32, String, i32, i32)>,
) -> Vec<(i32, String, String, String, i32, i32, f64)> {
    let book_map: HashMap<i32, (String, String, String, i32)> = books
        .into_iter()
        .map(|(id, title, author, genre, pages)| (id, (title, author, genre, pages)))
        .collect();
    let mut ratings: HashMap<i32, Vec<i32>> = HashMap::new();
    for (_sid, bid, _reader, _pages, rating) in sessions {
        ratings.entry(bid).or_default().push(rating);
    }
    let mut ans = Vec::new();
    for (bid, rs) in ratings {
        if rs.len() < 5 {
            continue;
        }
        let max_r = *rs.iter().max().unwrap();
        let min_r = *rs.iter().min().unwrap();
        if max_r < 4 || min_r > 2 {
            continue;
        }
        let extreme = rs.iter().filter(|&&r| r <= 2 || r >= 4).count();
        let score = (extreme as f64 / rs.len() as f64 * 100.0).round() / 100.0;
        if score < 0.6 {
            continue;
        }
        if let Some((title, author, genre, pages)) = book_map.get(&bid) {
            ans.push((
                bid,
                title.clone(),
                author.clone(),
                genre.clone(),
                *pages,
                max_r - min_r,
                score,
            ));
        }
    }
    ans.sort_by(|a, b| {
        b.6.partial_cmp(&a.6)
            .unwrap()
            .then(b.1.cmp(&a.1))
    });
    ans
}

fn main() {
    println!("{:?}", find_polarized_books(vec![], vec![]));
}

#[cfg(test)]
mod tests {
    use super::find_polarized_books;

    #[test]
    fn example() {
        let books = vec![
            (1, "The Great Gatsby".into(), "F. Scott".into(), "Fiction".into(), 180),
            (2, "To Kill a Mockingbird".into(), "Harper Lee".into(), "Fiction".into(), 281),
            (3, "1984".into(), "George Orwell".into(), "Dystopian".into(), 328),
            (4, "Pride and Prejudice".into(), "Jane Austen".into(), "Romance".into(), 432),
            (5, "The Catcher in the Rye".into(), "J.D. Salinger".into(), "Fiction".into(), 277),
        ];
        let sessions = vec![
            (1, 1, "Alice".into(), 50, 5),
            (2, 1, "Bob".into(), 60, 1),
            (3, 1, "Carol".into(), 40, 4),
            (4, 1, "David".into(), 30, 2),
            (5, 1, "Emma".into(), 45, 5),
            (6, 2, "Frank".into(), 80, 4),
            (7, 2, "Grace".into(), 70, 4),
            (8, 2, "Henry".into(), 90, 5),
            (9, 2, "Ivy".into(), 60, 4),
            (10, 2, "Jack".into(), 75, 4),
            (11, 3, "Kate".into(), 100, 2),
            (12, 3, "Liam".into(), 120, 1),
            (13, 3, "Mia".into(), 80, 2),
            (14, 3, "Noah".into(), 90, 1),
            (15, 3, "Olivia".into(), 110, 4),
            (16, 3, "Paul".into(), 95, 5),
            (17, 4, "Quinn".into(), 150, 3),
            (18, 4, "Ruby".into(), 140, 3),
            (19, 5, "Sam".into(), 80, 1),
            (20, 5, "Tara".into(), 70, 2),
        ];
        let ans = find_polarized_books(books, sessions);
        assert_eq!(ans.len(), 2);
        assert_eq!(ans[0].0, 1);
        assert_eq!(ans[1].0, 3);
        assert!((ans[0].6 - 1.0).abs() < 1e-9);
    }
}
