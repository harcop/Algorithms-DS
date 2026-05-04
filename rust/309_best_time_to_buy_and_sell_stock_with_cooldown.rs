/// LeetCode #309 - Best Time to Buy and Sell Stock with Cooldown
fn max_profit(prices: Vec<i32>) -> i32 {
    let mut hold = i32::MIN / 4;
    let mut sold = 0;
    let mut rest = 0;
    for p in prices {
        let prev_hold = hold;
        let prev_sold = sold;
        hold = hold.max(rest - p);
        sold = prev_hold + p;
        rest = rest.max(prev_sold);
    }
    sold.max(rest)
}

fn main() {
    println!("{}", max_profit(vec![1, 2, 3, 0, 2]));
}

#[cfg(test)]
mod tests {
    use super::max_profit;

    #[test]
    fn example_one() {
        assert_eq!(max_profit(vec![1, 2, 3, 0, 2]), 3);
    }

    #[test]
    fn example_two() {
        assert_eq!(max_profit(vec![1]), 0);
    }
}
