/// LeetCode #1251 - Average Selling Price (SQL; Rust analogue)

fn round2(x: f64) -> f64 {
    (x * 100.0).round() / 100.0
}

use std::collections::{HashMap, HashSet};

fn average_selling_price(
    prices: Vec<(i32, String, String, i32)>,
    units: Vec<(i32, String, i32)>,
) -> Vec<(i32, f64)> {
    let ids: HashSet<i32> = prices.iter().map(|p| p.0).collect();
    let mut acc: HashMap<i32, (i64, i64)> = HashMap::new();
    for (pid, date, u) in &units {
        for (ppid, start, end, price) in &prices {
            if ppid == pid && date.as_str() >= start.as_str() && date.as_str() <= end.as_str() {
                let e = acc.entry(*pid).or_insert((0, 0));
                e.0 += (*price as i64) * (*u as i64);
                e.1 += *u as i64;
            }
        }
    }
    let mut ans: Vec<(i32, f64)> = ids
        .into_iter()
        .map(|id| {
            let avg = acc
                .get(&id)
                .map(|(s, n)| if *n == 0 { 0.0 } else { round2(*s as f64 / *n as f64) })
                .unwrap_or(0.0);
            (id, avg)
        })
        .collect();
    ans.sort_by_key(|r| r.0);
    ans
}

fn main() {
    println!("ok");
}

#[cfg(test)]
mod tests {
    use super::average_selling_price;

    #[test]
    fn example() {
        let prices = vec![
            (1, "2019-02-17".into(), "2019-02-28".into(), 5),
            (1, "2019-03-01".into(), "2019-03-22".into(), 20),
            (2, "2019-02-01".into(), "2019-02-20".into(), 15),
            (2, "2019-02-21".into(), "2019-03-31".into(), 30),
        ];
        let units = vec![
            (1, "2019-02-25".into(), 100),
            (1, "2019-03-01".into(), 15),
            (2, "2019-02-10".into(), 200),
            (2, "2019-03-22".into(), 30),
        ];
        let got = average_selling_price(prices, units);
        assert_eq!(got[0].0, 1);
        assert!((got[0].1 - 6.96).abs() < 1e-9);
        assert_eq!(got[1].0, 2);
        assert!((got[1].1 - 16.96).abs() < 1e-9);
    }
}
