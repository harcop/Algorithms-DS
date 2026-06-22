/// LeetCode #2034 - Stock Price Fluctuation
use std::collections::{BTreeMap, HashMap};

pub struct StockPrice {
    prices: HashMap<i32, i32>,
    counts: BTreeMap<i32, i32>,
    last: i32,
}

impl StockPrice {
    fn new() -> Self {
        StockPrice {
            prices: HashMap::new(),
            counts: BTreeMap::new(),
            last: 0,
        }
    }

    fn add_count(&mut self, price: i32, delta: i32) {
        let entry = self.counts.entry(price).or_insert(0);
        *entry += delta;
        if *entry == 0 {
            self.counts.remove(&price);
        }
    }

    fn update(&mut self, timestamp: i32, price: i32) {
        if let Some(old) = self.prices.insert(timestamp, price) {
            self.add_count(old, -1);
        }
        self.add_count(price, 1);
        self.last = self.last.max(timestamp);
    }

    fn current(&self) -> i32 {
        self.prices[&self.last]
    }

    fn maximum(&self) -> i32 {
        *self.counts.keys().next_back().unwrap()
    }

    fn minimum(&self) -> i32 {
        *self.counts.keys().next().unwrap()
    }
}

fn main() {
    let mut stock = StockPrice::new();
    stock.update(1, 10);
    stock.update(2, 5);
    println!("{}", stock.current());
}

#[cfg(test)]
mod tests {
    use super::StockPrice;

    #[test]
    fn example_one() {
        let mut stock = StockPrice::new();
        stock.update(1, 10);
        stock.update(2, 5);
        assert_eq!(stock.current(), 5);
        assert_eq!(stock.maximum(), 10);
        stock.update(1, 3);
        assert_eq!(stock.maximum(), 5);
        stock.update(4, 2);
        assert_eq!(stock.minimum(), 2);
    }
}
