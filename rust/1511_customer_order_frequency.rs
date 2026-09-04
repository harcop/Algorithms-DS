/// LeetCode #1511 - Customer Order Frequency (SQL; Rust analogue)
use std::collections::HashMap;

fn customer_order_frequency(
    customers: Vec<(i32, String, String)>,
    product: Vec<(i32, String, i32)>,
    orders: Vec<(i32, i32, i32, String, i32)>,
) -> Vec<(i32, String)> {
    let price: HashMap<i32, i32> = product.into_iter().map(|(id, _, p)| (id, p)).collect();
    let names: HashMap<i32, String> = customers.into_iter().map(|(id, n, _)| (id, n)).collect();
    let mut spend: HashMap<i32, (i32, i32)> = HashMap::new();
    for (_, cid, pid, date, qty) in orders {
        if !date.starts_with("2020-") {
            continue;
        }
        let amount = qty * price[&pid];
        let e = spend.entry(cid).or_insert((0, 0));
        if date.starts_with("2020-06") {
            e.0 += amount;
        } else if date.starts_with("2020-07") {
            e.1 += amount;
        }
    }
    let mut ans: Vec<(i32, String)> = spend
        .into_iter()
        .filter(|(_, (j, l))| *j >= 100 && *l >= 100)
        .map(|(id, _)| (id, names[&id].clone()))
        .collect();
    ans.sort_by_key(|r| r.0);
    ans
}

fn main() {
    println!("{:?}", customer_order_frequency(vec![], vec![], vec![]));
}

#[cfg(test)]
mod tests {
    use super::customer_order_frequency;

    #[test]
    fn example() {
        let customers = vec![
            (1, "Winston".into(), "USA".into()),
            (2, "Jonathan".into(), "Peru".into()),
            (3, "Moustafa".into(), "Egypt".into()),
        ];
        let product = vec![
            (10, "LC Phone".into(), 300),
            (20, "LC T-Shirt".into(), 10),
            (30, "LC Book".into(), 45),
            (40, "LC Keychain".into(), 2),
        ];
        let orders = vec![
            (1, 1, 10, "2020-06-10".into(), 1),
            (2, 1, 20, "2020-07-01".into(), 1),
            (3, 1, 30, "2020-07-08".into(), 2),
            (4, 2, 10, "2020-06-15".into(), 2),
            (5, 2, 40, "2020-07-01".into(), 10),
            (6, 3, 20, "2020-06-24".into(), 2),
            (7, 3, 30, "2020-06-25".into(), 2),
            (9, 3, 30, "2020-05-08".into(), 3),
        ];
        assert_eq!(
            customer_order_frequency(customers, product, orders),
            vec![(1, "Winston".into())]
        );
    }
}
