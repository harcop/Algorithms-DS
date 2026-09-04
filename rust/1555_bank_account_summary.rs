/// LeetCode #1555 - Bank Account Summary (SQL; Rust analogue)
use std::collections::HashMap;

fn bank_account_summary(
    users: Vec<(i32, String, i32)>,
    transactions: Vec<(i32, i32, i32, i32, String)>,
) -> Vec<(i32, String, i32, String)> {
    let mut credit: HashMap<i32, i32> = HashMap::new();
    let mut names: HashMap<i32, String> = HashMap::new();
    for (id, name, c) in users {
        credit.insert(id, c);
        names.insert(id, name);
    }
    for (_, paid_by, paid_to, amount, _) in transactions {
        *credit.entry(paid_by).or_insert(0) -= amount;
        *credit.entry(paid_to).or_insert(0) += amount;
    }
    let mut ans: Vec<(i32, String, i32, String)> = credit
        .into_iter()
        .map(|(id, c)| {
            let breached = if c < 0 { "Yes" } else { "No" };
            (id, names[&id].clone(), c, breached.into())
        })
        .collect();
    ans.sort_by_key(|r| r.0);
    ans
}

fn main() {
    println!("{:?}", bank_account_summary(vec![], vec![]));
}

#[cfg(test)]
mod tests {
    use super::bank_account_summary;

    #[test]
    fn example() {
        let users = vec![
            (1, "Moustafa".into(), 100),
            (2, "Jonathan".into(), 200),
            (3, "Winston".into(), 10000),
            (4, "Luis".into(), 800),
        ];
        let transactions = vec![
            (1, 1, 3, 400, "2020-08-01".into()),
            (2, 3, 2, 500, "2020-08-02".into()),
            (3, 2, 1, 200, "2020-08-03".into()),
        ];
        assert_eq!(
            bank_account_summary(users, transactions),
            vec![
                (1, "Moustafa".into(), -100, "Yes".into()),
                (2, "Jonathan".into(), 500, "No".into()),
                (3, "Winston".into(), 9900, "No".into()),
                (4, "Luis".into(), 800, "No".into()),
            ]
        );
    }
}
