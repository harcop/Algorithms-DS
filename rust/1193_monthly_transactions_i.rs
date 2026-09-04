/// LeetCode #1193 - Monthly Transactions I (SQL; Rust analogue)

fn year_month(s: &str) -> String {
    let p: Vec<&str> = s
        .split(|c: char| !c.is_ascii_digit())
        .filter(|x| !x.is_empty())
        .collect();
    format!("{}-{:02}", p[0], p[1].parse::<i32>().unwrap())
}

use std::collections::HashMap;

fn monthly_transactions(
    transactions: Vec<(i32, String, String, i32, String)>,
) -> Vec<(String, String, i32, i32, i32, i32)> {
    let mut acc: HashMap<(String, String), (i32, i32, i32, i32)> = HashMap::new();
    for (_, country, state, amount, date) in transactions {
        let m = year_month(&date);
        let e = acc.entry((m, country)).or_insert((0, 0, 0, 0));
        e.0 += 1;
        e.2 += amount;
        if state == "approved" {
            e.1 += 1;
            e.3 += amount;
        }
    }
    let mut ans: Vec<(String, String, i32, i32, i32, i32)> = acc
        .into_iter()
        .map(|((m, c), (tc, ac, ta, aa))| (m, c, tc, ac, ta, aa))
        .collect();
    ans.sort();
    ans
}

fn main() {
    println!("ok");
}

#[cfg(test)]
mod tests {
    use super::monthly_transactions;

    #[test]
    fn example() {
        let transactions = vec![
            (121, "US".into(), "approved".into(), 1000, "2018-12-18".into()),
            (122, "US".into(), "declined".into(), 2000, "2018-12-19".into()),
            (123, "US".into(), "approved".into(), 2000, "2019-01-01".into()),
            (124, "DE".into(), "approved".into(), 2000, "2019-01-07".into()),
        ];
        assert_eq!(
            monthly_transactions(transactions),
            vec![
                ("2018-12".into(), "US".into(), 2, 1, 3000, 1000),
                ("2019-01".into(), "DE".into(), 1, 1, 2000, 2000),
                ("2019-01".into(), "US".into(), 1, 1, 2000, 2000),
            ]
        );
    }
}
