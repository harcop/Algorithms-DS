/// LeetCode #121 - Best Time to Buy and Sell Stock
fn max_profit(prices: Vec<i32>) -> i32 {
    let mut min_price = i32::MAX;
    let mut best = 0;
    for p in prices {
        min_price = min_price.min(p);
        best = best.max(p - min_price);
    }
    best
}

fn main() {
    println!("{}", max_profit(vec![7, 1, 5, 3, 6, 4]));
}

#[cfg(test)]
mod tests {
    use super::max_profit;

    #[test]
    fn example_one() {
        assert_eq!(max_profit(vec![7, 1, 5, 3, 6, 4]), 5);
    }

    #[test]
    fn example_two() {
        assert_eq!(max_profit(vec![7, 6, 4, 3, 1]), 0);
    }
}
