/// LeetCode #1831 - Maximum Transaction Each Day (SQL; Rust analogue)
use std::collections::HashMap;

fn maximum_transaction_each_day(transactions: Vec<(i32, String, i32)>) -> Vec<i32> {
    let mut best: HashMap<String, i32> = HashMap::new();
    for (_, day, amount) in &transactions {
        let key = day.split_whitespace().next().unwrap().to_string();
        best.entry(key)
            .and_modify(|m| *m = (*m).max(*amount))
            .or_insert(*amount);
    }
    let mut ans: Vec<i32> = transactions
        .into_iter()
        .filter(|(_, day, amount)| {
            let key = day.split_whitespace().next().unwrap();
            best.get(key).copied() == Some(*amount)
        })
        .map(|(id, _, _)| id)
        .collect();
    ans.sort();
    ans
}

fn main() {
    let transactions = vec![
        (8, "2021-4-3 15:57:28".into(), 57),
        (9, "2021-4-28 08:47:25".into(), 21),
        (1, "2021-4-29 13:28:30".into(), 58),
        (5, "2021-4-28 16:39:59".into(), 40),
        (6, "2021-4-29 23:39:28".into(), 58),
    ];
    println!("{:?}", maximum_transaction_each_day(transactions));
}

#[cfg(test)]
mod tests {
    use super::maximum_transaction_each_day;

    #[test]
    fn example_one() {
        let transactions = vec![
            (8, "2021-4-3 15:57:28".into(), 57),
            (9, "2021-4-28 08:47:25".into(), 21),
            (1, "2021-4-29 13:28:30".into(), 58),
            (5, "2021-4-28 16:39:59".into(), 40),
            (6, "2021-4-29 23:39:28".into(), 58),
        ];
        assert_eq!(maximum_transaction_each_day(transactions), vec![1, 5, 6, 8]);
    }
}
