/// LeetCode #1596 - The Most Frequently Ordered Products for Each Customer (SQL; Rust analogue)
use std::collections::HashMap;

fn most_frequent_products(
    _customers: Vec<(i32, String)>,
    orders: Vec<(i32, String, i32, i32)>,
    products: Vec<(i32, String, i32)>,
) -> Vec<(i32, i32, String)> {
    let names: HashMap<i32, String> = products.into_iter().map(|(id, n, _)| (id, n)).collect();
    let mut cnt: HashMap<(i32, i32), i32> = HashMap::new();
    for (_, _, cid, pid) in orders {
        *cnt.entry((cid, pid)).or_insert(0) += 1;
    }
    let mut max_c: HashMap<i32, i32> = HashMap::new();
    for (&(cid, _), &c) in &cnt {
        max_c
            .entry(cid)
            .and_modify(|m| *m = (*m).max(c))
            .or_insert(c);
    }
    let mut ans: Vec<(i32, i32, String)> = cnt
        .into_iter()
        .filter(|((cid, _), c)| *c == max_c[cid])
        .map(|((cid, pid), _)| (cid, pid, names[&pid].clone()))
        .collect();
    ans.sort_by_key(|r| (r.0, r.1));
    ans
}

fn main() {
    println!("{:?}", most_frequent_products(vec![], vec![], vec![]));
}

#[cfg(test)]
mod tests {
    use super::most_frequent_products;

    #[test]
    fn example() {
        let customers = vec![
            (1, "Alice".into()),
            (2, "Bob".into()),
            (3, "Tom".into()),
            (4, "Jerry".into()),
            (5, "John".into()),
        ];
        let orders = vec![
            (1, "2020-07-31".into(), 1, 1),
            (2, "2020-07-30".into(), 2, 2),
            (3, "2020-08-29".into(), 3, 3),
            (4, "2020-07-29".into(), 4, 1),
            (5, "2020-06-10".into(), 1, 2),
            (6, "2020-08-01".into(), 2, 1),
            (7, "2020-08-01".into(), 3, 3),
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
            most_frequent_products(customers, orders, products),
            vec![
                (1, 2, "mouse".into()),
                (2, 1, "keyboard".into()),
                (2, 2, "mouse".into()),
                (2, 3, "screen".into()),
                (3, 3, "screen".into()),
                (4, 1, "keyboard".into()),
            ]
        );
    }
}
