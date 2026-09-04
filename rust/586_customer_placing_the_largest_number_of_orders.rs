/// LeetCode #586 - Customer Placing the Largest Number of Orders (SQL; Rust analogue)
use std::collections::HashMap;

fn largest_order_customer(orders: Vec<(i32, i32)>) -> i32 {
    let mut cnt: HashMap<i32, i32> = HashMap::new();
    for (_, customer) in orders {
        *cnt.entry(customer).or_insert(0) += 1;
    }
    cnt.into_iter().max_by_key(|(c, n)| (*n, -c)).unwrap().0
}

fn main() {
    println!("ok");
}

#[cfg(test)]
mod tests {
    use super::largest_order_customer;

    #[test]
    fn example() {
        let orders = vec![(1, 1), (2, 2), (3, 3), (4, 3)];
        assert_eq!(largest_order_customer(orders), 3);
    }
}
