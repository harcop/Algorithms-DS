/// LeetCode #3166 - Calculate Parking Fees and Duration (SQL; Rust analogue)
use std::collections::BTreeMap;

/// row: (lot_id, car_id, entry_secs, exit_secs, fee_paid)
fn parking_stats(
    txs: Vec<(i32, i32, i64, i64, f64)>,
) -> Vec<(i32, f64, f64, i32)> {
    let mut by_car: BTreeMap<i32, Vec<(i32, i64, f64)>> = BTreeMap::new();
    for (lot, car, entry, exit, fee) in txs {
        by_car
            .entry(car)
            .or_default()
            .push((lot, exit - entry, fee));
    }
    let mut ans = Vec::new();
    for (car, rows) in by_car {
        let total_fee: f64 = rows.iter().map(|r| r.2).sum();
        let total_secs: i64 = rows.iter().map(|r| r.1).sum();
        let avg = (total_fee / (total_secs as f64 / 3600.0) * 100.0).round() / 100.0;
        let mut lot_time: BTreeMap<i32, i64> = BTreeMap::new();
        for &(lot, secs, _) in &rows {
            *lot_time.entry(lot).or_insert(0) += secs;
        }
        let most_time_lot = lot_time
            .into_iter()
            .max_by_key(|&(lot, secs)| (secs, -lot))
            .map(|(lot, _)| lot)
            .unwrap();
        ans.push((car, total_fee, avg, most_time_lot));
    }
    ans
}

fn main() {
    let txs = vec![
        (1, 1001, 0, 9000, 5.0),
        (1, 1001, 100000, 100000 + 6300, 3.0),
        (2, 1001, 9900, 9900 + 4500, 6.0),
        (2, 1002, 3600, 3600 + 9000, 4.0),
        (3, 1001, 200000, 200000 + 7200, 4.0),
        (3, 1002, 150000, 150000 + 7200, 2.0),
    ];
    println!("{:?}", parking_stats(txs));
}

#[cfg(test)]
mod tests {
    use super::parking_stats;

    #[test]
    fn example() {
        // durations match the problem statement hours
        let txs = vec![
            (1, 1001, 0, 2 * 3600 + 1800, 5.0),       // 2.5h lot1
            (1, 1001, 100000, 100000 + 6300, 3.0),    // 1.75h lot1
            (2, 1001, 200000, 200000 + 4500, 6.0),    // 1.25h lot2
            (2, 1002, 0, 9000, 4.0),                  // 2.5h lot2
            (3, 1001, 300000, 300000 + 7200, 4.0),    // 2h lot3
            (3, 1002, 100000, 100000 + 7200, 2.0),    // 2h lot3
        ];
        let ans = parking_stats(txs);
        assert_eq!(ans.len(), 2);
        assert_eq!(ans[0].0, 1001);
        assert!((ans[0].1 - 18.0).abs() < 1e-9);
        assert!((ans[0].2 - 2.40).abs() < 1e-9);
        assert_eq!(ans[0].3, 1);
        assert_eq!(ans[1].0, 1002);
        assert!((ans[1].1 - 6.0).abs() < 1e-9);
        assert!((ans[1].2 - 1.33).abs() < 1e-9);
        assert_eq!(ans[1].3, 2);
    }
}
