/// LeetCode #1581 - Customer Who Visited but Did Not Make Any Transactions (SQL; Rust analogue)
use std::collections::{HashMap, HashSet};

fn visits_without_transactions(
    visits: Vec<(i32, i32)>,
    transactions: Vec<(i32, i32, i32)>,
) -> Vec<(i32, i32)> {
    let with_tx: HashSet<i32> = transactions.into_iter().map(|(_, vid, _)| vid).collect();
    let mut cnt: HashMap<i32, i32> = HashMap::new();
    for (vid, cid) in visits {
        if !with_tx.contains(&vid) {
            *cnt.entry(cid).or_insert(0) += 1;
        }
    }
    cnt.into_iter().collect()
}

fn main() {
    println!("{:?}", visits_without_transactions(vec![], vec![]));
}

#[cfg(test)]
mod tests {
    use super::visits_without_transactions;

    #[test]
    fn example() {
        let visits = vec![
            (1, 23),
            (2, 9),
            (4, 30),
            (5, 54),
            (6, 96),
            (7, 54),
            (8, 54),
        ];
        let transactions = vec![
            (2, 5, 310),
            (3, 5, 300),
            (9, 5, 200),
            (12, 1, 910),
            (13, 2, 970),
        ];
        let mut got = visits_without_transactions(visits, transactions);
        got.sort_by_key(|r| r.0);
        assert_eq!(got, vec![(30, 1), (54, 2), (96, 1)]);
    }
}
