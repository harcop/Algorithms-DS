/// LeetCode #1205 - Monthly Transactions II (SQL; Rust analogue)

fn year_month(s: &str) -> String {
    let p: Vec<&str> = s
        .split(|c: char| !c.is_ascii_digit())
        .filter(|x| !x.is_empty())
        .collect();
    format!("{}-{:02}", p[0], p[1].parse::<i32>().unwrap())
}

use std::collections::HashMap;

fn monthly_transactions_ii(
    transactions: Vec<(i32, String, String, i32, String)>,
    chargebacks: Vec<(i32, String)>,
) -> Vec<(String, String, i32, i32, i32, i32)> {
    let tx: HashMap<i32, (String, i32)> = transactions
        .iter()
        .map(|(id, country, _, amount, _)| (*id, (country.clone(), *amount)))
        .collect();
    let mut acc: HashMap<(String, String), (i32, i32, i32, i32)> = HashMap::new();
    for (_, country, state, amount, date) in &transactions {
        if state == "approved" {
            let m = year_month(date);
            let e = acc.entry((m, country.clone())).or_insert((0, 0, 0, 0));
            e.0 += 1;
            e.1 += amount;
        }
    }
    for (tid, date) in chargebacks {
        if let Some((country, amount)) = tx.get(&tid) {
            let m = year_month(&date);
            let e = acc.entry((m, country.clone())).or_insert((0, 0, 0, 0));
            e.2 += 1;
            e.3 += amount;
        }
    }
    let mut ans: Vec<(String, String, i32, i32, i32, i32)> = acc
        .into_iter()
        .filter(|(_, v)| v.1 != 0 || v.3 != 0)
        .map(|((m, c), (ac, aa, cc, ca))| (m, c, ac, aa, cc, ca))
        .collect();
    ans.sort();
    ans
}

fn main() {
    println!("ok");
}

#[cfg(test)]
mod tests {
    use super::monthly_transactions_ii;

    #[test]
    fn example() {
        let transactions = vec![
            (101, "US".into(), "approved".into(), 1000, "2019-05-18".into()),
            (102, "US".into(), "declined".into(), 2000, "2019-05-19".into()),
            (103, "US".into(), "approved".into(), 3000, "2019-06-10".into()),
            (104, "US".into(), "declined".into(), 4000, "2019-06-13".into()),
            (105, "US".into(), "approved".into(), 5000, "2019-06-15".into()),
        ];
        let chargebacks = vec![
            (102, "2019-05-29".into()),
            (101, "2019-06-30".into()),
            (105, "2019-09-18".into()),
        ];
        assert_eq!(
            monthly_transactions_ii(transactions, chargebacks),
            vec![
                ("2019-05".into(), "US".into(), 1, 1000, 1, 2000),
                ("2019-06".into(), "US".into(), 2, 8000, 1, 1000),
                ("2019-09".into(), "US".into(), 0, 0, 1, 5000),
            ]
        );
    }
}
