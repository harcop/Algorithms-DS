/// LeetCode #3230 - Customer Purchasing Behavior Analysis (SQL; Rust analogue)
/// transactions: (transaction_id, customer_id, product_id, transaction_date, amount)
/// products: (product_id, category, price)
fn customer_purchasing_behavior(
    transactions: Vec<(i32, i32, i32, String, f64)>,
    products: Vec<(i32, String, f64)>,
) -> Vec<(i32, f64, i32, i32, f64, String, f64)> {
    use std::collections::{HashMap, HashSet};
    let cat: HashMap<i32, String> = products.into_iter().map(|(id, c, _)| (id, c)).collect();

    struct Agg {
        total: f64,
        count: i32,
        categories: HashSet<String>,
        cat_cnt: HashMap<String, i32>,
        cat_last: HashMap<String, String>,
    }

    let mut by_cust: HashMap<i32, Agg> = HashMap::new();
    for (_, customer_id, product_id, date, amount) in transactions {
        let category = cat.get(&product_id).cloned().unwrap_or_default();
        let e = by_cust.entry(customer_id).or_insert(Agg {
            total: 0.0,
            count: 0,
            categories: HashSet::new(),
            cat_cnt: HashMap::new(),
            cat_last: HashMap::new(),
        });
        e.total += amount;
        e.count += 1;
        e.categories.insert(category.clone());
        *e.cat_cnt.entry(category.clone()).or_insert(0) += 1;
        let last = e.cat_last.entry(category).or_insert(String::new());
        if date > *last {
            *last = date;
        }
    }

    let mut ans: Vec<(i32, f64, i32, i32, f64, String, f64)> = by_cust
        .into_iter()
        .map(|(customer_id, a)| {
            let top_category = a
                .cat_cnt
                .iter()
                .max_by(|(c1, n1), (c2, n2)| {
                    n1.cmp(n2)
                        .then_with(|| a.cat_last[*c1].cmp(&a.cat_last[*c2]))
                })
                .map(|(c, _)| c.clone())
                .unwrap_or_default();
            let total = (a.total * 100.0).round() / 100.0;
            let avg = ((a.total / a.count as f64) * 100.0).round() / 100.0;
            let loyalty = ((a.count as f64 * 10.0 + a.total / 100.0) * 100.0).round() / 100.0;
            (
                customer_id,
                total,
                a.count,
                a.categories.len() as i32,
                avg,
                top_category,
                loyalty,
            )
        })
        .collect();
    ans.sort_by(|a, b| {
        b.6.partial_cmp(&a.6)
            .unwrap()
            .then_with(|| a.0.cmp(&b.0))
    });
    ans
}

fn main() {
    let tx = vec![
        (1, 101, 1, "2023-01-01".into(), 100.0),
        (2, 101, 2, "2023-01-15".into(), 150.0),
        (3, 102, 1, "2023-01-01".into(), 100.0),
        (4, 102, 3, "2023-01-22".into(), 200.0),
        (5, 101, 3, "2023-02-10".into(), 200.0),
    ];
    let products = vec![
        (1, "A".into(), 100.0),
        (2, "B".into(), 150.0),
        (3, "C".into(), 200.0),
    ];
    println!("{:?}", customer_purchasing_behavior(tx, products));
}

#[cfg(test)]
mod tests {
    use super::customer_purchasing_behavior;

    #[test]
    fn example() {
        let tx = vec![
            (1, 101, 1, "2023-01-01".into(), 100.0),
            (2, 101, 2, "2023-01-15".into(), 150.0),
            (3, 102, 1, "2023-01-01".into(), 100.0),
            (4, 102, 3, "2023-01-22".into(), 200.0),
            (5, 101, 3, "2023-02-10".into(), 200.0),
        ];
        let products = vec![
            (1, "A".into(), 100.0),
            (2, "B".into(), 150.0),
            (3, "C".into(), 200.0),
        ];
        assert_eq!(
            customer_purchasing_behavior(tx, products),
            vec![
                (101, 450.0, 3, 3, 150.0, "C".into(), 34.5),
                (102, 300.0, 2, 2, 150.0, "C".into(), 23.0),
            ]
        );
    }
}
