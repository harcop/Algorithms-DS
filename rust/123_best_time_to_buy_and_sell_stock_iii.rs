/// LeetCode #123 - Best Time to Buy and Sell Stock III (at most 2 transactions)
fn max_profit(prices: Vec<i32>) -> i32 {
    let mut buy1 = i32::MAX;
    let mut profit1 = 0;
    let mut buy2 = i32::MAX;
    let mut profit2 = 0;
    for p in prices {
        buy1 = buy1.min(p);
        profit1 = profit1.max(p - buy1);
        buy2 = buy2.min(p - profit1);
        profit2 = profit2.max(p - buy2);
    }
    profit2
}

fn main() {
    println!("{}", max_profit(vec![3, 3, 5, 0, 0, 3, 1, 4]));
}

#[cfg(test)]
mod tests {
    use super::max_profit;

    #[test]
    fn example_one() {
        assert_eq!(max_profit(vec![3, 3, 5, 0, 0, 3, 1, 4]), 6);
    }

    #[test]
    fn example_two() {
        assert_eq!(max_profit(vec![1, 2, 3, 4, 5]), 4);
    }
}
