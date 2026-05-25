/// LeetCode #1357 - Apply Discount Every N Orders

use std::collections::{HashMap, HashSet};

struct MPSMC {
    n: i32,
    discount: i32,
    prices: HashMap<i32, i32>,
    members: HashSet<i32>,
    orders: i32,
}

impl MPSMC {
    fn new(n: i32, discount: i32, products: Vec<i32>, prices: Vec<i32>) -> Self {
        let prices = products.into_iter().zip(prices).collect();
        Self { n, discount, prices, members: HashSet::new(), orders: 0 }
    }

    fn add_member(&mut self, id: i32) {
        self.members.insert(id);
    }

    fn purchase(&mut self, id: i32, product_id: i32, price: i32) -> i32 {
        self.orders += 1;
        let mut charge = *self.prices.get(&product_id).unwrap_or(&price);
        if self.members.contains(&id) && self.orders % self.n == 0 {
            charge = charge * (100 - self.discount) / 100;
        }
        charge
    }
}

fn main() {
    let mut s = MPSMC::new(3, 50, vec![1, 2], vec![100, 200]);
    s.add_member(1);
    println!("{}", s.purchase(1, 1, 100));
}

#[cfg(test)]
mod tests {
    use super::MPSMC;

    #[test]
    fn example_one() {
        let mut s = MPSMC::new(3, 50, vec![1, 2], vec![100, 200]);
        s.add_member(1);
        assert_eq!(s.purchase(1, 1, 100), 100);
        assert_eq!(s.purchase(1, 2, 200), 200);
        assert_eq!(s.purchase(1, 1, 100), 50);
    }
}
