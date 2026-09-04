/// LeetCode #1565 - Unique Orders and Customers Per Month (SQL; Rust analogue)
use std::collections::{BTreeMap, HashSet};

fn unique_orders_customers(orders: Vec<(i32, String, i32, i32)>) -> Vec<(String, i32, i32)> {
    let mut by_month: BTreeMap<String, (i32, HashSet<i32>)> = BTreeMap::new();
    for (_, date, cid, invoice) in orders {
        if invoice > 20 {
            let month = date[..7].to_string();
            let e = by_month.entry(month).or_insert((0, HashSet::new()));
            e.0 += 1;
            e.1.insert(cid);
        }
    }
    by_month
        .into_iter()
        .map(|(m, (oc, cs))| (m, oc, cs.len() as i32))
        .collect()
}

fn main() {
    println!("{:?}", unique_orders_customers(vec![]));
}

#[cfg(test)]
mod tests {
    use super::unique_orders_customers;

    #[test]
    fn example() {
        let orders = vec![
            (1, "2020-09-15".into(), 1, 30),
            (2, "2020-09-17".into(), 2, 90),
            (3, "2020-10-06".into(), 3, 20),
            (4, "2020-10-20".into(), 3, 21),
            (5, "2020-11-10".into(), 1, 10),
            (6, "2020-11-21".into(), 2, 15),
            (7, "2020-12-01".into(), 4, 55),
            (8, "2020-12-03".into(), 4, 77),
            (9, "2021-01-07".into(), 3, 31),
            (10, "2021-01-15".into(), 2, 20),
        ];
        assert_eq!(
            unique_orders_customers(orders),
            vec![
                ("2020-09".into(), 2, 2),
                ("2020-10".into(), 1, 1),
                ("2020-12".into(), 2, 1),
                ("2021-01".into(), 1, 1),
            ]
        );
    }
}
