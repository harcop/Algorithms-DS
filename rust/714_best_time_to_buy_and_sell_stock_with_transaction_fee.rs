/// LeetCode #714 - Best Time to Buy and Sell Stock with Transaction Fee
fn max_profit(prices: Vec<i32>, fee: i32) -> i32 {
    let mut hold = -prices[0];
    let mut cash = 0i32;
    for i in 1..prices.len() {
        cash = cash.max(hold + prices[i] - fee);
        hold = hold.max(cash - prices[i]);
    }
    cash
}

fn main() {
    println!("{}", max_profit(vec![1,3,2,8,4,9], 2));
}

#[cfg(test)]
mod tests {
    use super::max_profit;

    #[test]
    fn example_one() {
        assert_eq!(max_profit(vec![1,3,2,8,4,9], 2), 8);
    }
}
