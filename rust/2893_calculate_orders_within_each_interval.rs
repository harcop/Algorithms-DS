/// LeetCode #2893 - Calculate Orders Within Each Interval (SQL; Rust analogue)
fn calculate_orders(orders: Vec<(i32, i32)>) -> Vec<(i32, i64)> {
    let mut totals = std::collections::BTreeMap::new();
    for (minute, order_count) in orders {
        let interval = (minute + 5) / 6;
        *totals.entry(interval).or_insert(0i64) += order_count as i64;
    }
    totals.into_iter().collect()
}

fn main() {
    let orders = vec![
        (1, 0),
        (2, 2),
        (3, 4),
        (4, 6),
        (5, 1),
        (6, 4),
        (7, 1),
        (8, 2),
        (9, 4),
        (10, 1),
        (11, 4),
        (12, 6),
    ];
    println!("{:?}", calculate_orders(orders));
}

#[cfg(test)]
mod tests {
    use super::calculate_orders;

    #[test]
    fn example() {
        let orders = vec![
            (1, 0),
            (2, 2),
            (3, 4),
            (4, 6),
            (5, 1),
            (6, 4),
            (7, 1),
            (8, 2),
            (9, 4),
            (10, 1),
            (11, 4),
            (12, 6),
        ];
        assert_eq!(calculate_orders(orders), vec![(1, 17), (2, 18)]);
    }
}
