/// LeetCode #2985 - Calculate Compressed Mean (SQL; Rust analogue)
fn compressed_mean(orders: Vec<(i32, i32, i32)>) -> f64 {
    // (order_id, item_count, order_occurrences)
    let mut total_items = 0i64;
    let mut total_orders = 0i64;
    for (_, item_count, occ) in orders {
        total_items += item_count as i64 * occ as i64;
        total_orders += occ as i64;
    }
    let mean = total_items as f64 / total_orders as f64;
    (mean * 100.0).round() / 100.0
}

fn main() {
    println!(
        "{:.2}",
        compressed_mean(vec![(10, 1, 500), (11, 2, 1000), (12, 3, 800), (13, 4, 1000)])
    );
}

#[cfg(test)]
mod tests {
    use super::compressed_mean;

    #[test]
    fn example() {
        let v = compressed_mean(vec![(10, 1, 500), (11, 2, 1000), (12, 3, 800), (13, 4, 1000)]);
        assert!((v - 2.70).abs() < 1e-9);
    }
}
