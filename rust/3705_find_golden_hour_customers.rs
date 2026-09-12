/// LeetCode #3705 - Find Golden Hour Customers (SQL; Rust analogue)
use std::collections::HashMap;

/// order: (order_id, customer_id, hour, minute, order_rating Option)
fn find_golden_hour_customers(
    orders: Vec<(i32, i32, i32, i32, Option<i32>)>,
) -> Vec<(i32, i32, i32, f64)> {
    struct Agg {
        total: i32,
        peak: i32,
        rated: i32,
        rating_sum: i32,
    }
    let mut map: HashMap<i32, Agg> = HashMap::new();
    for (_oid, cid, hour, minute, rating) in orders {
        let mins = hour * 60 + minute;
        let is_peak = (11 * 60 <= mins && mins <= 14 * 60)
            || (18 * 60 <= mins && mins <= 21 * 60);
        let e = map.entry(cid).or_insert(Agg {
            total: 0,
            peak: 0,
            rated: 0,
            rating_sum: 0,
        });
        e.total += 1;
        if is_peak {
            e.peak += 1;
        }
        if let Some(r) = rating {
            e.rated += 1;
            e.rating_sum += r;
        }
    }
    let mut ans: Vec<(i32, i32, i32, f64)> = map
        .into_iter()
        .filter_map(|(cid, a)| {
            if a.total < 3 {
                return None;
            }
            let peak_pct = ((a.peak as f64 / a.total as f64) * 100.0).round() as i32;
            if peak_pct < 60 {
                return None;
            }
            if a.rated * 2 < a.total {
                return None;
            }
            if a.rated == 0 {
                return None;
            }
            let avg = (a.rating_sum as f64 / a.rated as f64 * 100.0).round() / 100.0;
            if avg < 4.0 {
                return None;
            }
            Some((cid, a.total, peak_pct, avg))
        })
        .collect();
    ans.sort_by(|a, b| {
        b.3
            .partial_cmp(&a.3)
            .unwrap()
            .then_with(|| b.0.cmp(&a.0))
    });
    ans
}

fn main() {
    println!("{:?}", find_golden_hour_customers(vec![]));
}

#[cfg(test)]
mod tests {
    use super::find_golden_hour_customers;

    #[test]
    fn example() {
        let orders = vec![
            (1, 101, 12, 30, Some(5)),
            (2, 101, 19, 15, Some(4)),
            (3, 101, 13, 45, Some(5)),
            (4, 101, 20, 30, None),
            (5, 102, 11, 30, Some(4)),
            (6, 102, 12, 0, Some(3)),
            (7, 102, 15, 30, None),
            (8, 103, 19, 0, Some(5)),
            (9, 103, 20, 45, Some(4)),
            (10, 103, 18, 30, Some(5)),
            (11, 104, 10, 0, Some(3)),
            (12, 104, 9, 30, Some(2)),
            (13, 104, 16, 0, Some(3)),
            (14, 105, 12, 15, Some(4)),
            (15, 105, 13, 0, Some(5)),
            (16, 105, 11, 45, Some(4)),
        ];
        let ans = find_golden_hour_customers(orders);
        assert_eq!(
            ans,
            vec![
                (103, 3, 100, 4.67),
                (101, 4, 100, 4.67),
                (105, 3, 100, 4.33),
            ]
        );
    }
}
