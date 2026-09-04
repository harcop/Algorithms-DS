/// LeetCode #1843 - Suspicious Bank Accounts (SQL; Rust analogue)
use std::collections::{HashMap, HashSet};

fn year_month(day: &str) -> i32 {
    let date = day.split_whitespace().next().unwrap();
    let mut p = date.split('-');
    let y: i32 = p.next().unwrap().parse().unwrap();
    let m: i32 = p.next().unwrap().parse().unwrap();
    y * 12 + m
}

fn suspicious_bank_accounts(
    accounts: Vec<(i32, i32)>,
    transactions: Vec<(i32, i32, String, i32, String)>,
) -> Vec<i32> {
    let max_income: HashMap<i32, i32> = accounts.into_iter().collect();
    let mut monthly: HashMap<(i32, i32), i32> = HashMap::new();
    for (_tid, account_id, typ, amount, day) in transactions {
        if typ == "Creditor" {
            let ym = year_month(&day);
            *monthly.entry((account_id, ym)).or_insert(0) += amount;
        }
    }
    let mut over: HashMap<i32, Vec<i32>> = HashMap::new();
    for ((account_id, ym), sum) in monthly {
        if let Some(&cap) = max_income.get(&account_id) {
            if sum > cap {
                over.entry(account_id).or_default().push(ym);
            }
        }
    }
    let mut ans: Vec<i32> = Vec::new();
    let mut seen = HashSet::new();
    for (account_id, mut months) in over {
        months.sort();
        months.dedup();
        for w in months.windows(2) {
            if w[1] == w[0] + 1 && seen.insert(account_id) {
                ans.push(account_id);
            }
        }
    }
    ans.sort();
    ans
}

fn main() {
    println!("{:?}", suspicious_bank_accounts(vec![], vec![]));
}

#[cfg(test)]
mod tests {
    use super::suspicious_bank_accounts;

    #[test]
    fn example_one() {
        let accounts = vec![(3, 21000), (4, 10400)];
        let transactions = vec![
            (2, 3, "Creditor".into(), 107100, "2021-06-02 11:38:14".into()),
            (4, 4, "Creditor".into(), 10400, "2021-06-20 12:39:18".into()),
            (11, 4, "Debtor".into(), 58800, "2021-07-23 12:41:55".into()),
            (1, 4, "Creditor".into(), 49300, "2021-05-03 16:11:04".into()),
            (15, 3, "Debtor".into(), 75500, "2021-05-23 14:40:20".into()),
            (10, 3, "Creditor".into(), 102100, "2021-06-15 10:37:16".into()),
            (14, 4, "Creditor".into(), 56300, "2021-07-21 12:12:25".into()),
            (19, 4, "Debtor".into(), 101100, "2021-05-09 15:21:49".into()),
            (8, 3, "Creditor".into(), 64900, "2021-07-26 15:09:56".into()),
            (7, 3, "Creditor".into(), 90900, "2021-06-14 11:23:07".into()),
        ];
        assert_eq!(suspicious_bank_accounts(accounts, transactions), vec![3]);
    }
}
