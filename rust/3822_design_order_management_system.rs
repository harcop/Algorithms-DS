/// LeetCode #3822 - Design Order Management System (premium)
use std::collections::HashMap;

struct OrderManagementSystem {
    orders: HashMap<i32, (String, i32)>,
    t: HashMap<(String, i32), Vec<i32>>,
}

impl OrderManagementSystem {
    fn new() -> Self {
        Self {
            orders: HashMap::new(),
            t: HashMap::new(),
        }
    }

    fn add_order(&mut self, order_id: i32, order_type: String, price: i32) {
        self.orders
            .insert(order_id, (order_type.clone(), price));
        self.t
            .entry((order_type, price))
            .or_default()
            .push(order_id);
    }

    fn modify_order(&mut self, order_id: i32, new_price: i32) {
        if let Some((order_type, old_price)) = self.orders.get(&order_id).cloned() {
            self.orders
                .insert(order_id, (order_type.clone(), new_price));
            if let Some(v) = self.t.get_mut(&(order_type.clone(), old_price)) {
                if let Some(pos) = v.iter().position(|&x| x == order_id) {
                    v.remove(pos);
                }
            }
            self.t
                .entry((order_type, new_price))
                .or_default()
                .push(order_id);
        }
    }

    fn cancel_order(&mut self, order_id: i32) {
        if let Some((order_type, price)) = self.orders.remove(&order_id) {
            if let Some(v) = self.t.get_mut(&(order_type, price)) {
                if let Some(pos) = v.iter().position(|&x| x == order_id) {
                    v.remove(pos);
                }
            }
        }
    }

    fn get_orders_at_price(&self, order_type: String, price: i32) -> Vec<i32> {
        self.t
            .get(&(order_type, price))
            .cloned()
            .unwrap_or_default()
    }
}

fn main() {
    let mut oms = OrderManagementSystem::new();
    oms.add_order(1, "buy".into(), 1);
    println!("{:?}", oms.get_orders_at_price("buy".into(), 1));
}

#[cfg(test)]
mod tests {
    use super::OrderManagementSystem;

    #[test]
    fn example1() {
        let mut oms = OrderManagementSystem::new();
        oms.add_order(1, "buy".into(), 1);
        oms.add_order(2, "buy".into(), 1);
        oms.add_order(3, "sell".into(), 2);
        let mut got = oms.get_orders_at_price("buy".into(), 1);
        got.sort_unstable();
        assert_eq!(got, vec![1, 2]);
        oms.modify_order(1, 3);
        oms.modify_order(2, 1);
        assert_eq!(oms.get_orders_at_price("buy".into(), 1), vec![2]);
        oms.cancel_order(3);
        oms.cancel_order(2);
        assert_eq!(
            oms.get_orders_at_price("buy".into(), 1),
            Vec::<i32>::new()
        );
    }
}
