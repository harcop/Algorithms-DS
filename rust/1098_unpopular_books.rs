/// LeetCode #1098 - Unpopular Books (SQL; Rust analogue)
use std::collections::HashMap;

fn unpopular_books(
    books: Vec<(i32, String, String)>,
    orders: Vec<(i32, i32, i32, String)>,
) -> Vec<(i32, String)> {
    let mut qty: HashMap<i32, i32> = HashMap::new();
    for (_, bid, q, date) in orders {
        if date.as_str() >= "2018-06-23" {
            *qty.entry(bid).or_insert(0) += q;
        }
    }
    let mut ans = Vec::new();
    for (id, name, avail) in books {
        if avail.as_str() >= "2019-05-23" {
            continue;
        }
        if *qty.get(&id).unwrap_or(&0) < 10 {
            ans.push((id, name));
        }
    }
    ans.sort();
    ans
}

fn main() {
    println!("ok");
}

#[cfg(test)]
mod tests {
    use super::unpopular_books;

    #[test]
    fn example() {
        let books = vec![
            (1, "Kalila And Demna".into(), "2010-01-01".into()),
            (2, "28 Letters".into(), "2012-05-12".into()),
            (3, "The Hobbit".into(), "2019-06-10".into()),
            (4, "13 Reasons Why".into(), "2019-06-01".into()),
            (5, "The Hunger Games".into(), "2008-09-21".into()),
        ];
        let orders = vec![
            (1, 1, 2, "2018-07-26".into()),
            (2, 1, 1, "2018-11-05".into()),
            (3, 3, 8, "2019-06-11".into()),
            (4, 4, 6, "2019-06-05".into()),
            (5, 4, 5, "2019-06-20".into()),
            (6, 5, 9, "2009-02-02".into()),
            (7, 5, 8, "2010-04-13".into()),
        ];
        assert_eq!(
            unpopular_books(books, orders),
            vec![
                (1, "Kalila And Demna".into()),
                (2, "28 Letters".into()),
                (5, "The Hunger Games".into()),
            ]
        );
    }
}
