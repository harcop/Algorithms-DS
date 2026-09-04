/// LeetCode #1336 - Number of Transactions per Visit (SQL; Rust analogue)
use std::collections::HashMap;

fn transactions_per_visit(
    visits: Vec<(i32, String)>,
    transactions: Vec<(i32, String, i32)>,
) -> Vec<(i32, i32)> {
    let mut tx: HashMap<(i32, String), i32> = HashMap::new();
    for (uid, date, _) in transactions {
        *tx.entry((uid, date)).or_insert(0) += 1;
    }
    let mut freq: HashMap<i32, i32> = HashMap::new();
    let mut max_cnt = 0;
    for (uid, date) in visits {
        let c = *tx.get(&(uid, date)).unwrap_or(&0);
        *freq.entry(c).or_insert(0) += 1;
        max_cnt = max_cnt.max(c);
    }
    (0..=max_cnt)
        .map(|n| (n, *freq.get(&n).unwrap_or(&0)))
        .collect()
}

fn main() {
    println!("{:?}", transactions_per_visit(vec![], vec![]));
}

#[cfg(test)]
mod tests {
    use super::transactions_per_visit;

    #[test]
    fn example() {
        let visits = vec![
            (1, "2020-01-01".into()),
            (2, "2020-01-02".into()),
            (12, "2020-01-01".into()),
            (19, "2020-01-03".into()),
            (1, "2020-01-02".into()),
            (2, "2020-01-03".into()),
            (1, "2020-01-04".into()),
            (7, "2020-01-11".into()),
            (9, "2020-01-25".into()),
            (8, "2020-01-28".into()),
        ];
        let transactions = vec![
            (1, "2020-01-02".into(), 120),
            (2, "2020-01-03".into(), 22),
            (7, "2020-01-11".into(), 232),
            (1, "2020-01-04".into(), 7),
            (9, "2020-01-25".into(), 33),
            (9, "2020-01-25".into(), 66),
            (8, "2020-01-28".into(), 1),
            (9, "2020-01-25".into(), 99),
        ];
        assert_eq!(
            transactions_per_visit(visits, transactions),
            vec![(0, 4), (1, 5), (2, 0), (3, 1)]
        );
    }
}
