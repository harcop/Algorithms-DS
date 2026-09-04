/// LeetCode #1867 - Orders With Maximum Quantity Above Average (SQL; Rust analogue)
use std::collections::HashMap;

fn imbalanced_orders(orders_details: Vec<(i32, i32, i32)>) -> Vec<i32> {
    let mut by_order: HashMap<i32, Vec<i32>> = HashMap::new();
    for (order_id, _product_id, quantity) in orders_details {
        by_order.entry(order_id).or_default().push(quantity);
    }
    let stats: Vec<(i32, i32, f64)> = by_order
        .into_iter()
        .map(|(id, qs)| {
            let max_q = *qs.iter().max().unwrap();
            let avg = qs.iter().sum::<i32>() as f64 / qs.len() as f64;
            (id, max_q, avg)
        })
        .collect();
    let max_avg = stats.iter().map(|t| t.2).fold(f64::NEG_INFINITY, f64::max);
    let mut ans: Vec<i32> = stats
        .into_iter()
        .filter(|(_, max_q, _)| (*max_q as f64) > max_avg)
        .map(|(id, _, _)| id)
        .collect();
    ans.sort();
    ans
}

fn main() {
    println!("{:?}", imbalanced_orders(vec![]));
}

#[cfg(test)]
mod tests {
    use super::imbalanced_orders;

    #[test]
    fn example_one() {
        let orders = vec![
            (1, 1, 12),
            (1, 2, 10),
            (1, 3, 15),
            (2, 1, 8),
            (2, 4, 4),
            (2, 5, 6),
            (3, 3, 5),
            (3, 4, 18),
            (4, 5, 2),
            (4, 6, 8),
            (5, 7, 9),
            (5, 8, 9),
            (3, 9, 20),
            (2, 9, 4),
        ];
        assert_eq!(imbalanced_orders(orders), vec![1, 3]);
    }
}
