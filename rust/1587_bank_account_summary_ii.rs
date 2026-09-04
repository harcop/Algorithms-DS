/// LeetCode #1587 - Bank Account Summary II (SQL; Rust analogue)
use std::collections::HashMap;

fn bank_account_summary_ii(
    users: Vec<(i32, String)>,
    transactions: Vec<(i32, i32, i32, String)>,
) -> Vec<(String, i32)> {
    let names: HashMap<i32, String> = users.into_iter().collect();
    let mut bal: HashMap<i32, i32> = HashMap::new();
    for (_, account, amount, _) in transactions {
        *bal.entry(account).or_insert(0) += amount;
    }
    bal.into_iter()
        .filter(|(_, b)| *b > 10000)
        .map(|(id, b)| (names[&id].clone(), b))
        .collect()
}

fn main() {
    println!("{:?}", bank_account_summary_ii(vec![], vec![]));
}

#[cfg(test)]
mod tests {
    use super::bank_account_summary_ii;

    #[test]
    fn example() {
        let users = vec![
            (900001, "Alice".into()),
            (900002, "Bob".into()),
            (900003, "Charlie".into()),
        ];
        let transactions = vec![
            (1, 900001, 7000, "2020-08-01".into()),
            (2, 900001, 7000, "2020-08-02".into()),
            (3, 900001, -3000, "2020-08-03".into()),
            (4, 900002, 1000, "2020-08-26".into()),
            (5, 900003, 6000, "2020-08-07".into()),
            (6, 900003, 6000, "2020-08-17".into()),
            (7, 900003, -4000, "2020-08-24".into()),
        ];
        assert_eq!(
            bank_account_summary_ii(users, transactions),
            vec![("Alice".into(), 11000)]
        );
    }
}
