/// LeetCode #122 - Best Time to Buy and Sell Stock II
fn max_profit(prices: Vec<i32>) -> i32 {
    let mut total = 0;
    for i in 1..prices.len() {
        total += (prices[i] - prices[i - 1]).max(0);
    }
    total
}

fn main() {
    println!("{}", max_profit(vec![7, 1, 5, 3, 6, 4]));
}

#[cfg(test)]
mod tests {
    use super::max_profit;

    #[test]
    fn example_one() {
        assert_eq!(max_profit(vec![7, 1, 5, 3, 6, 4]), 7);
    }

    #[test]
    fn example_two() {
        assert_eq!(max_profit(vec![1, 2, 3, 4, 5]), 4);
    }
}
