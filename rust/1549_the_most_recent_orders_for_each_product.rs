/// LeetCode #1549 - The Most Recent Orders for Each Product (SQL; Rust analogue)
use std::collections::HashMap;

fn most_recent_orders(
    _customers: Vec<(i32, String)>,
    orders: Vec<(i32, String, i32, i32)>,
    products: Vec<(i32, String, i32)>,
) -> Vec<(String, i32, i32, String)> {
    let names: HashMap<i32, String> = products.into_iter().map(|(id, n, _)| (id, n)).collect();
    let mut latest: HashMap<i32, String> = HashMap::new();
    for (_, date, _, pid) in &orders {
        latest
            .entry(*pid)
            .and_modify(|d| {
                if date > d {
                    *d = date.clone();
                }
            })
            .or_insert(date.clone());
    }
    let mut ans = Vec::new();
    for (oid, date, _, pid) in orders {
        if latest.get(&pid) == Some(&date) {
            if let Some(name) = names.get(&pid) {
                ans.push((name.clone(), pid, oid, date));
            }
        }
    }
    ans.sort_by(|a, b| a.0.cmp(&b.0).then(a.1.cmp(&b.1)).then(a.2.cmp(&b.2)));
    ans
}

fn main() {
    println!("{:?}", most_recent_orders(vec![], vec![], vec![]));
}

#[cfg(test)]
mod tests {
    use super::most_recent_orders;

    #[test]
    fn example() {
        let customers = vec![
            (1, "Winston".into()),
            (2, "Jonathan".into()),
            (3, "Annabelle".into()),
            (4, "Marwan".into()),
            (5, "Khaled".into()),
        ];
        let orders = vec![
            (1, "2020-07-31".into(), 1, 1),
            (2, "2020-07-30".into(), 2, 2),
            (3, "2020-08-29".into(), 3, 3),
            (4, "2020-07-29".into(), 4, 1),
            (5, "2020-06-10".into(), 1, 2),
            (6, "2020-08-01".into(), 2, 1),
            (7, "2020-08-01".into(), 3, 1),
            (8, "2020-08-03".into(), 1, 2),
            (9, "2020-08-07".into(), 2, 3),
            (10, "2020-07-15".into(), 1, 2),
        ];
        let products = vec![
            (1, "keyboard".into(), 120),
            (2, "mouse".into(), 80),
            (3, "screen".into(), 600),
            (4, "hard disk".into(), 450),
        ];
        assert_eq!(
            most_recent_orders(customers, orders, products),
            vec![
                ("keyboard".into(), 1, 6, "2020-08-01".into()),
                ("keyboard".into(), 1, 7, "2020-08-01".into()),
                ("mouse".into(), 2, 8, "2020-08-03".into()),
                ("screen".into(), 3, 3, "2020-08-29".into()),
            ]
        );
    }
}
