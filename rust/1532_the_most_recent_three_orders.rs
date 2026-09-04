/// LeetCode #1532 - The Most Recent Three Orders (SQL; Rust analogue)
use std::collections::HashMap;

fn most_recent_three_orders(
    customers: Vec<(i32, String)>,
    orders: Vec<(i32, String, i32, i32)>,
) -> Vec<(String, i32, i32, String)> {
    let names: HashMap<i32, String> = customers.into_iter().collect();
    let mut by_cust: HashMap<i32, Vec<(String, i32)>> = HashMap::new();
    for (oid, date, cid, _) in orders {
        by_cust.entry(cid).or_default().push((date, oid));
    }
    let mut ans = Vec::new();
    for (cid, mut rows) in by_cust {
        rows.sort_by(|a, b| b.0.cmp(&a.0));
        rows.truncate(3);
        let name = names[&cid].clone();
        for (date, oid) in rows {
            ans.push((name.clone(), cid, oid, date));
        }
    }
    ans.sort_by(|a, b| a.0.cmp(&b.0).then(a.1.cmp(&b.1)).then(b.3.cmp(&a.3)));
    ans
}

fn main() {
    println!("{:?}", most_recent_three_orders(vec![], vec![]));
}

#[cfg(test)]
mod tests {
    use super::most_recent_three_orders;

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
            (1, "2020-07-31".into(), 1, 30),
            (2, "2020-07-30".into(), 2, 40),
            (3, "2020-07-31".into(), 3, 70),
            (4, "2020-07-29".into(), 4, 100),
            (5, "2020-06-10".into(), 1, 1010),
            (6, "2020-08-01".into(), 2, 102),
            (7, "2020-08-01".into(), 3, 111),
            (8, "2020-08-03".into(), 1, 99),
            (9, "2020-08-07".into(), 2, 32),
            (10, "2020-07-15".into(), 1, 2),
        ];
        assert_eq!(
            most_recent_three_orders(customers, orders),
            vec![
                ("Annabelle".into(), 3, 7, "2020-08-01".into()),
                ("Annabelle".into(), 3, 3, "2020-07-31".into()),
                ("Jonathan".into(), 2, 9, "2020-08-07".into()),
                ("Jonathan".into(), 2, 6, "2020-08-01".into()),
                ("Jonathan".into(), 2, 2, "2020-07-30".into()),
                ("Marwan".into(), 4, 4, "2020-07-29".into()),
                ("Winston".into(), 1, 8, "2020-08-03".into()),
                ("Winston".into(), 1, 1, "2020-07-31".into()),
                ("Winston".into(), 1, 10, "2020-07-15".into()),
            ]
        );
    }
}
