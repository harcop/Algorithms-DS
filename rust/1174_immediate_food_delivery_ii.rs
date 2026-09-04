/// LeetCode #1174 - Immediate Food Delivery II (SQL; Rust analogue)

fn round2(x: f64) -> f64 {
    (x * 100.0).round() / 100.0
}

use std::collections::HashMap;

fn immediate_food_delivery_ii(delivery: Vec<(i32, i32, String, String)>) -> f64 {
    let mut first: HashMap<i32, (String, String)> = HashMap::new();
    for (_, cid, order, pref) in delivery {
        first
            .entry(cid)
            .and_modify(|e| {
                if order < e.0 {
                    *e = (order.clone(), pref.clone());
                }
            })
            .or_insert((order, pref));
    }
    if first.is_empty() {
        return 0.0;
    }
    let imm = first.values().filter(|(o, p)| o == p).count() as f64;
    round2(imm / first.len() as f64 * 100.0)
}

fn main() {
    println!("ok");
}

#[cfg(test)]
mod tests {
    use super::immediate_food_delivery_ii;

    #[test]
    fn example() {
        let delivery = vec![
            (1, 1, "2019-08-01".into(), "2019-08-02".into()),
            (2, 2, "2019-08-02".into(), "2019-08-02".into()),
            (3, 1, "2019-08-11".into(), "2019-08-12".into()),
            (4, 3, "2019-08-24".into(), "2019-08-24".into()),
            (5, 3, "2019-08-21".into(), "2019-08-22".into()),
            (6, 2, "2019-08-11".into(), "2019-08-13".into()),
            (7, 4, "2019-08-09".into(), "2019-08-09".into()),
        ];
        assert!((immediate_food_delivery_ii(delivery) - 50.0).abs() < 1e-9);
    }
}
