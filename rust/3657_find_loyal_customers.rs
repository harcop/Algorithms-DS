/// LeetCode #3657 - Find Loyal Customers (SQL; Rust analogue)
use std::collections::HashMap;

fn find_loyal_customers(transactions: Vec<(i32, i32, i32, f64, String)>) -> Vec<i32> {
    // (transaction_id, customer_id, day_offset, amount, type)
    // day_offset is days since epoch for simplicity
    struct Agg {
        purchases: i32,
        refunds: i32,
        min_day: i32,
        max_day: i32,
    }
    let mut map: HashMap<i32, Agg> = HashMap::new();
    for (_tid, cid, day, _amt, ty) in transactions {
        let e = map.entry(cid).or_insert(Agg {
            purchases: 0,
            refunds: 0,
            min_day: day,
            max_day: day,
        });
        if ty == "purchase" {
            e.purchases += 1;
        } else {
            e.refunds += 1;
        }
        e.min_day = e.min_day.min(day);
        e.max_day = e.max_day.max(day);
    }
    let mut ans: Vec<i32> = map
        .into_iter()
        .filter(|(_, a)| {
            let total = a.purchases + a.refunds;
            a.purchases >= 3
                && a.max_day - a.min_day >= 30
                && (a.refunds as f64) / (total as f64) < 0.2
        })
        .map(|(cid, _)| cid)
        .collect();
    ans.sort_unstable();
    ans
}

fn main() {
    println!("{:?}", find_loyal_customers(vec![]));
}

#[cfg(test)]
mod tests {
    use super::find_loyal_customers;

    #[test]
    fn example() {
        // day offsets approximating the sample dates
        let txs = vec![
            (1, 101, 5, 150.0, "purchase".into()),
            (2, 101, 15, 200.0, "purchase".into()),
            (3, 101, 41, 180.0, "purchase".into()),
            (4, 101, 51, 250.0, "purchase".into()),
            (5, 102, 10, 100.0, "purchase".into()),
            (6, 102, 12, 120.0, "purchase".into()),
            (7, 102, 15, 80.0, "refund".into()),
            (8, 102, 18, 90.0, "refund".into()),
            (9, 102, 46, 130.0, "purchase".into()),
            (10, 103, 1, 500.0, "purchase".into()),
            (11, 103, 2, 450.0, "purchase".into()),
            (12, 103, 3, 400.0, "purchase".into()),
            (13, 104, 1, 200.0, "purchase".into()),
            (14, 104, 32, 250.0, "purchase".into()),
            (15, 104, 46, 300.0, "purchase".into()),
            (16, 104, 61, 350.0, "purchase".into()),
            (17, 104, 70, 280.0, "purchase".into()),
            (18, 104, 75, 100.0, "refund".into()),
        ];
        assert_eq!(find_loyal_customers(txs), vec![101, 104]);
    }
}
