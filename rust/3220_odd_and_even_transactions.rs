/// LeetCode #3220 - Odd and Even Transactions (SQL; Rust analogue)
/// transactions: (transaction_id, amount, transaction_date)
fn odd_and_even_transactions(transactions: Vec<(i32, i32, String)>) -> Vec<(String, i32, i32)> {
    use std::collections::BTreeMap;
    let mut map: BTreeMap<String, (i32, i32)> = BTreeMap::new();
    for (_, amount, date) in transactions {
        let e = map.entry(date).or_insert((0, 0));
        if amount % 2 == 1 {
            e.0 += amount;
        } else {
            e.1 += amount;
        }
    }
    map.into_iter()
        .map(|(date, (odd, even))| (date, odd, even))
        .collect()
}

fn main() {
    let tx = vec![
        (1, 150, "2024-07-01".into()),
        (2, 200, "2024-07-01".into()),
        (3, 75, "2024-07-01".into()),
    ];
    println!("{:?}", odd_and_even_transactions(tx));
}

#[cfg(test)]
mod tests {
    use super::odd_and_even_transactions;

    #[test]
    fn example() {
        let tx = vec![
            (1, 150, "2024-07-01".into()),
            (2, 200, "2024-07-01".into()),
            (3, 75, "2024-07-01".into()),
            (4, 300, "2024-07-02".into()),
            (5, 50, "2024-07-02".into()),
            (6, 120, "2024-07-03".into()),
        ];
        assert_eq!(
            odd_and_even_transactions(tx),
            vec![
                ("2024-07-01".into(), 75, 350),
                ("2024-07-02".into(), 0, 350),
                ("2024-07-03".into(), 0, 120),
            ]
        );
    }
}
